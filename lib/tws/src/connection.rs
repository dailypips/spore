use std::io;
use std::net::SocketAddr;

use futures::Future;

use bytes::{Bytes, BytesMut};
use tokio::net::TcpStream;
use tokio_io::{codec::length_delimited, codec::Framed, AsyncRead, AsyncWrite};
//use clear_on_drop::ClearOnDrop;
use failure::Error;
use futures;
use futures::Sink;
use futures::Stream;
use std;
use tokio;

pub type RespConnection = Framed<TcpStream, length_delimited::Framed>;

pub fn connect(addr: &SocketAddr) -> impl Future<Item = RespConnection, Error = io::Error> {
    TcpStream::connect(addr).map(move |socket| length_delimited::Framed::new(socket))
}

#[derive(Debug, Fail)]
enum SessionError {
    #[fail(display = "peer has not signed its x25519 key")]
    MissingPeerCertificate,

    #[fail(display = "peer has different identity than expected")]
    IdentityMismatch,
}

pub struct Session<S: AsyncRead + AsyncWrite + Send> {
    /*peer: Identity,
    peer_certificate: ValidCertificateChain,
    noise: snow::Session,*/
    framed: SessionState<S>,
}

impl<S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Send> Session<S> {
    pub fn peer(&self) -> &Identity {
        &self.peer
    }
    pub fn peer_certificate(&self) -> &ValidCertificateChain {
        &self.peer_certificate
    }
}

pub struct SessionBuilder {
    certificate: Certificate,
    x25519: ClearOnDrop<Vec<u8>>,
}

impl SessionBuilder {
    pub fn new() -> Self {
        Self {
            certificate: Certificate::default(),
            x25519: ClearOnDrop::new(Vec::default()),
        }
    }

    pub fn certificate(mut self, certificate: Certificate) -> Self {
        self.certificate = certificate;
        self
    }

    pub fn x25519(mut self, x25519: ClearOnDrop<Vec<u8>>) -> Self {
        self.x25519 = x25519;
        self
    }

    pub fn connect(
        self,
        expected_identity: Identity,
        addr: &std::net::SocketAddr,
    ) -> impl Future<Item = Session<tokio::net::TcpStream>, Error = Error> + Send {
        let mycertificate = self.certificate;
        let myx25519 = self.x25519;

        let hello = TcpStream::connect(&addr)
            .map_err(|e|{
                debug!("tcp error: {}", e);
                e.into()
            })
            // tcp established
            .and_then(move |socket| {
                let params: NoiseParams = "Noise_IX_25519_AESGCM_SHA256".parse().unwrap();
                let mut noise = NoiseBuilder::new(params)
                    .local_private_key(&*myx25519)
                    .prologue("carrier has arrived".as_bytes())
                    .build_initiator()
                    .expect("building noise session");

                let framed = length_delimited::Builder::new()
                    .new_framed(socket);

                let mut buf = BytesMut::from([0;65535].as_ref());

                // -> e, s
                let len = match noise.write_message(&[], &mut buf) {
                    Ok(v) => v,
                    Err(e) => return Box::new(futures::future::err(e))
                        as Box<Future<Item=_, Error=Error>+ Send>,
                };
                trace!("[ini] sending  -> e, s as {} bytes total", len);
                buf.truncate(len);
                let fut = framed.send(buf.freeze())
                    .map(|v|(noise,v))
                    .map_err(|e|e.into());
                Box::new(fut)
            })
            // send done, receive one frame
            .and_then(move |(noise, framed)|{
                framed.into_future()
                    .map(|(frame, framed)|(noise, frame, framed))
                    .map_err(|e| e.0.into())
            })
            // <- e, ee, se, s, es, certificate
            .and_then(move |(mut noise, frame, framed)|{
                if let None = frame {
                    return Box::new(futures::future::err(
                            std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "eof").into()))
                        as Box<Future<Item=_, Error=Error>+ Send>;
                };
                let frame = frame.unwrap();
                let mut buf = BytesMut::from([0;65535].as_ref());
                let pl = match noise.read_message(&frame, &mut buf) {
                    Ok(v) => v,
                    Err(e) => return Box::new(futures::future::err(e))
                        as Box<Future<Item=_, Error=Error>+ Send>,
                };
                trace!("[ini] received <- e, ee, se, s, es  with payload {}", pl);

                let peercert = match Certificate(buf[..pl].to_vec()).to_valid_chain() {
                    Ok(v)  => v,
                    Err(e) => return Box::new(futures::future::err(e.into()))
                };

                let real_identity = {
                    let remote_static = match noise.get_remote_static() {
                        None => return Box::new(futures::future::err(SessionError::MissingPeerCertificate.into())),
                        Some(v) => v,
                    };

                    match peercert.identity_for_x25519(&remote_static) {
                        None => return Box::new(futures::future::err(SessionError::MissingPeerCertificate.into())),
                        Some(id) => id,
                    }
                };

                if real_identity.ct_eq(&expected_identity).unwrap_u8() == 0 {
                    return Box::new(futures::future::err(SessionError::IdentityMismatch.into()));
                }

                //IX noise is finished, still need to send my identity
                let mut noise = match noise.into_transport_mode() {
                    Ok(v) => v,
                    Err(e) => return Box::new(futures::future::err(e))
                        as Box<Future<Item=_, Error=Error>+ Send>,
                };

                // -> identity
                let len = match noise.write_message(&mycertificate.0, &mut buf) {
                    Ok(v) => v,
                    Err(e) => return Box::new(futures::future::err(e))
                        as Box<Future<Item=_, Error=Error>+ Send>,
                };
                trace!("[ini] sending  identity {} bytes total", len);
                buf.truncate(len);
                let fut = framed.send(buf.freeze())
                    .map(|v|(noise, real_identity, peercert, v))
                    .map_err(|e|e.into());
                Box::new(fut)
            })
            // send done
            .and_then(move |(noise, peer, peercert, framed)|{
                Ok(Session{
                    peer: peer,
                    peer_certificate:   peercert,
                    noise:  noise,
                    framed: SessionState::Idle(framed),
                })
            });
        hello
    }

    pub fn accept<S: 'static>(
        self,
        socket: S,
    ) -> impl Future<Item = Session<S>, Error = Error> + Send
    where
        S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Send,
    {
        let mycertificate = self.certificate;
        let myx25519 = self.x25519;

        let params: NoiseParams = "Noise_IX_25519_AESGCM_SHA256".parse().unwrap();
        let mut noise = NoiseBuilder::new(params)
            .local_private_key(&*myx25519)
            .prologue("carrier has arrived".as_bytes())
            .build_responder()
            .expect("building noise session");

        let framed = length_delimited::Builder::new().new_framed::<_, Bytes>(socket);

        // receive one frame
        framed.into_future()
            .map_err(|e| e.0.into())
            .and_then(move |(frame, framed)|{
                match frame {
                    None => Box::new(futures::future::err(
                            std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "eof").into()))
                        as Box<Future<Item=_, Error=Error>+ Send>,
                    Some(frame) => {
                        // -> e, s
                        let mut buf = BytesMut::from([0;65535].as_ref());
                        let pl = match noise.read_message(&frame, &mut buf) {
                            Ok(v) => v,
                            Err(e) => return Box::new(futures::future::err(e))
                                as Box<Future<Item=_, Error=Error>+ Send>,
                        };
                        trace!("[rsp] received -> e, s with payload {}", pl);

                        // <- e, ee, se, s, es, certificate
                        let len = match noise.write_message(&mycertificate.0, &mut buf) {
                            Ok(v) => v,
                            Err(e) => return Box::new(futures::future::err(e))
                                as Box<Future<Item=_, Error=Error>+ Send>,
                        };
                        trace!("[rsp] sending  <- e, ee, se, s, es, identity {} bytes total", len);
                        buf.truncate(len);
                        let fut = framed.send(buf.freeze())
                            .map(|v|(noise,v))
                            .map_err(|e|e.into());
                        Box::new(fut)
                    }
                }
            })
            // send done, receive one frame
            .and_then(move |(noise, framed)|{
                framed.into_future()
                    .map(|(frame, framed)|(noise, frame, framed))
                    .map_err(|e| e.0.into())
            })
            // -> identity
            .and_then(move |(noise, frame, framed)|{
                match frame {
                    None => Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "eof").into()),
                    Some(frame) => {
                        let mut noise = noise.into_transport_mode()?;

                        let mut buf = BytesMut::from([0;65535].as_ref());
                        let pl = match noise.read_message(&frame, &mut buf) {
                            Ok(v)  => v,
                            Err(e) => return Err(e),
                        };

                        let payload = &buf[..pl];
                        trace!("[rsp] received certificate with payload {}", pl);


                        let peercert = match Certificate(buf[..pl].to_vec()).to_valid_chain() {
                            Ok(v)  => v,
                            Err(e) => return Err(e.into())
                        };

                        let real_identity = {
                            let remote_static = match noise.get_remote_static() {
                                None => return Err(SessionError::MissingPeerCertificate.into()),
                                Some(v) => v,
                            };

                            match peercert.identity_for_x25519(&remote_static) {
                                None => return Err(SessionError::MissingPeerCertificate.into()),
                                Some(id) => id,
                            }
                        };

                        Ok(Session{
                            peer: real_identity,
                            peer_certificate:   peercert,
                            noise:  noise,
                            framed: SessionState::Idle(framed),
                        })
                    }
                }
            })
    }
}

enum SessionState<S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Send> {
    Invalid,
    Sending(futures::sink::Send<length_delimited::Framed<S, Bytes>>),
    Idle(length_delimited::Framed<S, Bytes>),
}

impl<S> Sink for Session<S>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Send,
{
    type SinkItem = Vec<u8>;
    type SinkError = Error;

    fn start_send(
        &mut self,
        item: Self::SinkItem,
    ) -> futures::StartSend<Self::SinkItem, Self::SinkError> {
        match std::mem::replace(&mut self.framed, SessionState::Invalid) {
            SessionState::Idle(framed) => {
                let mut buf = BytesMut::from([0; 65535].as_ref());
                let len = match self.noise.write_message(&item, &mut buf) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                };
                trace!("sending crypted transport {} bytes total", len);
                buf.truncate(len);

                let fut = framed.send(buf.freeze());
                std::mem::replace(&mut self.framed, SessionState::Sending(fut));

                Ok(futures::AsyncSink::Ready)
            }
            SessionState::Sending(mut send) => match send.poll() {
                Ok(futures::Async::NotReady) => Ok(futures::AsyncSink::NotReady(item)),
                Ok(futures::Async::Ready(t)) => {
                    std::mem::replace(&mut self.framed, SessionState::Idle(t));
                    self.start_send(item)
                }
                Err(e) => Err(e.into()),
            },
            SessionState::Invalid => {
                unreachable!();
            }
        }
    }

    fn poll_complete(&mut self) -> futures::Poll<(), Self::SinkError> {
        use take_mut::scoped;
        scoped::scope(|scope| {
            let (mut framed, hole) = scope.take(&mut self.framed);
            match framed {
                SessionState::Idle(_) => {
                    hole.fill(framed);
                    Ok(futures::Async::Ready(()))
                }
                SessionState::Invalid => unreachable!(),
                SessionState::Sending(ref mut send) => match send.poll() {
                    Ok(futures::Async::NotReady) => Ok(futures::Async::NotReady),
                    Ok(futures::Async::Ready(t)) => {
                        hole.fill(SessionState::Idle(t));
                        Ok(futures::Async::Ready(()))
                    }
                    Err(e) => Err(e.into()),
                },
            }
        })
    }
}

impl<S> Stream for Session<S>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Send,
{
    type Item = BytesMut;
    type Error = Error;
    fn poll(&mut self) -> Result<futures::Async<Option<Self::Item>>, Self::Error> {
        match self.framed {
            SessionState::Invalid => unreachable!(),
            SessionState::Sending(_) => {
                //TODO poll the send?
                Ok(futures::Async::NotReady)
            }
            SessionState::Idle(ref mut framed) => match framed.poll() {
                Err(e) => Err(e.into()),
                Ok(futures::Async::NotReady) => Ok(futures::Async::NotReady),
                Ok(futures::Async::Ready(None)) => Ok(futures::Async::Ready(None)),
                Ok(futures::Async::Ready(Some(frame))) => {
                    let mut buf = BytesMut::from([0; 65535].as_ref());
                    let pl = match self.noise.read_message(&frame, &mut buf) {
                        Ok(v) => v,
                        Err(e) => return Err(e.into()),
                    };
                    trace!("transport received payload {}", pl);
                    buf.truncate(pl);
                    Ok(futures::Async::Ready(Some(buf)))
                }
            },
        }
    }
}

#[test]
fn loopback() {
    use futures::{Future, Sink, Stream};
    use generate_x25519;
    use length_delimited;
    use std::env;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use tokio::net::TcpListener;
    use tokio::net::TcpStream;
    use tokio::runtime::Runtime;
    use Secret;

    let mut rt = Runtime::new().unwrap();
    let handle = rt.executor();

    let server_secret = Secret::from_bytes(&mut [
        0xc4, 0x44, 0x49, 0xc5, 0x69, 0x7b, 0x32, 0x69, 0x19, 0x70, 0x3b, 0xac, 0x03, 0x1c, 0xae,
        0x9d, 0x61, 0xb1, 0x9d, 0xef, 0xfd, 0x5a, 0x60, 0xba, 0x84, 0x4a, 0xf4, 0x92, 0xec, 0x2c,
        0xf0, 0x0d,
    ]);
    let server_identity = server_secret.identity();
    let (server_x25519_secret, server_x25519_public) = generate_x25519();

    let client_secret = Secret::from_bytes(&mut [
        0x9d, 0x61, 0xb1, 0x9d, 0xef, 0xfd, 0x5a, 0x60, 0xba, 0x84, 0x4a, 0xf4, 0x92, 0xec, 0x2c,
        0xc4, 0x44, 0x49, 0xc5, 0x69, 0x7b, 0x32, 0x69, 0x19, 0x70, 0x3b, 0xac, 0x03, 0x1c, 0xae,
        0x7f, 0x60,
    ]);
    let client_identity = client_secret.identity();
    let (client_x25519_secret, client_x25519_public) = generate_x25519();

    let client_certificate = Certificate::new()
        .identity(&client_identity)
        .grant_access(&server_identity, &client_identity)
        .bind_x25519(&client_x25519_public)
        .sign(&client_secret)
        .unwrap();

    let server_certificate = Certificate::new()
        .identity(&server_identity)
        .bind_x25519(&server_x25519_public)
        .sign(&server_secret)
        .unwrap();

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 12345);
    debug!(
        "[server] listening  on {:?} with id {}",
        &addr,
        server_identity.public_id()
    );
    let handle_ = handle.clone();
    let server = TcpListener::bind(&addr).unwrap();
    let server = server
        .incoming()
        .for_each(move |socket: tokio::net::TcpStream| {
            debug!("[server] incomming connection");
            let session = SessionBuilder::new()
                .certificate(server_certificate.clone())
                .x25519(server_x25519_secret.clone())
                .accept(socket)
                .map_err(|e| error!("[server] accept: {}", e))
                .and_then(move |session| {
                    debug!(
                        "[server] connection established with peer: {}",
                        session.peer().public_id()
                    );

                    let clock = tokio::timer::Interval::new(
                        std::time::Instant::now(),
                        std::time::Duration::from_secs(1),
                    ).take(3)
                        .map_err(|_| ());

                    let fut = clock
                        .fold(session, |session, _| {
                            debug!("[server] tick");
                            session
                                .send(b"foobar".to_vec())
                                .map_err(|e| error!("{}", e))
                        })
                        .then(|_| Ok(()));
                    fut
                });
            handle_.spawn(session);
            Ok(())
        })
        .map_err(|e| error!("server: {}", e));
    handle.spawn(server);

    debug!(
        "[client] connecting to {:?} with id {}",
        &addr,
        client_identity.public_id()
    );
    let hello = SessionBuilder::new()
        .certificate(client_certificate.clone())
        .x25519(client_x25519_secret.clone())
        .connect(server_identity.clone(), &addr)
        .map_err(|e| error!("[client] connect: {}", e))
        .and_then(move |session| {
            debug!(
                "[client] connection established with peer: {}",
                session.peer().public_id()
            );

            let fut = session
                .for_each(|message| {
                    debug!("[client] msg: {:x?}", message);
                    Ok(())
                })
                .and_then(|_| {
                    debug!("[client] stream closed");
                    Ok(())
                })
                .map_err(|e| error!("[client] stream error: {}", e));
            fut
        });

    rt.block_on(hello).expect("run");
}

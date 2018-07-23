use bytes::Bytes;
use decoder::handshark::*;
use encoder::api;
use failure;
use failure_derive;
use futures::sink::Sink;
use futures::stream::Stream;
use futures::sync::mpsc;
use futures::sync::mpsc::{Receiver, SendError, Sender};
use futures::Future;
use std::convert::From;
use std::error;
use std::fmt;
use std::io;
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use tokio::net::ConnectFuture;
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio::timer::{Deadline, DeadlineError};
use tokio_io::codec::length_delimited;

type Packet = tokio_io::codec::length_delimited::Framed<tokio::net::TcpStream, bytes::Bytes>;

#[derive(Debug)]
pub enum OutgoingMessage {
    None,
}
#[derive(Debug)]
pub enum IncomingMessage {
    None,
}

#[derive(Debug)]
pub struct Session {
    addr: SocketAddr,
    stream: tokio_io::codec::length_delimited::Framed<tokio::net::TcpStream, bytes::Bytes>,
    pub version: i32,
    pub addr_or_time: String,
}

impl Session {
    pub fn server_version(&self) -> i32 {
        self.version
    }
}

pub struct SessionConfig {
    host: String,
    port: u16,
    client_id: u32,
}

const API_HEAD: &[u8] = b"API\0";
const VERSION_HEAD: &[u8] = b"v100..121";

#[derive(Debug)]
pub enum SessionError {
    //DeadlineError(DeadlineError<std::io::Error>),
    IoError(std::io::Error),
    SendError(SendError<IncomingMessage>),
    TimerError(tokio::timer::Error),
    Timeout,
}

impl fmt::Display for SessionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            SessionError::IoError(ref err) => write!(f, "IO error: {}", err),
            //SessionError::DeadlineError(ref err) => write!(f, "Parse error: {}", err),
            SessionError::TimerError(ref err) => write!(f, "Timer error {}", err),
            SessionError::Timeout => write!(f, "timeout"),
            SessionError::SendError(ref err) => write!(f, "SendError: {}", err),
        }
    }
}

impl error::Error for SessionError {
    fn description(&self) -> &str {
        // Both underlying errors already impl `Error`, so we defer to their
        // implementations.
        match *self {
            SessionError::IoError(ref err) => err.description(),
            //SessionError::DeadlineError(ref err) => err.description(),
            SessionError::TimerError(ref err) => err.description(),
            SessionError::Timeout => "timeout",
            SessionError::SendError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            // N.B. Both of these implicitly cast `err` from their concrete
            // types (either `&io::Error` or `&num::ParseIntError`)
            // to a trait object `&Error`. This works because both error types
            // implement `Error`.
            SessionError::IoError(ref err) => Some(err),
            SessionError::TimerError(ref err) => Some(err),
            SessionError::Timeout => None,
            //SessionError::DeadlineError(ref err) => Some(err),
            SessionError::SendError(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for SessionError {
    fn from(err: io::Error) -> SessionError {
        SessionError::IoError(err)
    }
}

impl From<DeadlineError<io::Error>> for SessionError {
    fn from(err: DeadlineError<io::Error>) -> SessionError {
        //SessionError::DeadlineError(err)
        if err.is_inner() {
            return SessionError::IoError(err.into_inner().unwrap());
        }
        if err.is_timer() {
            return SessionError::TimerError(err.into_timer().unwrap());
        }
        SessionError::Timeout
    }
}

pub fn decode(msg: bytes::BytesMut) -> IncomingMessage {
    IncomingMessage::None
}

pub fn encode(msg: OutgoingMessage) -> bytes::Bytes {
    bytes::BytesMut::new().freeze()
}

impl Session {
    pub fn connect(
        addr: SocketAddr,
        timeout_secs: u64,
    ) -> impl Future<Item = Self, Error = SessionError> {
        TcpStream::connect(&addr)
            .deadline(Instant::now() + Duration::from_secs(timeout_secs))
            .map_err(|e| ::std::convert::From::from(e))
            //.map_err(|e| SessionError::DeadlineError(e))
            .and_then(move |socket| {
                tokio::io::write_all(socket, API_HEAD)
                    .deadline(Instant::now() + Duration::from_secs(timeout_secs))
                    .map_err(|e| ::std::convert::From::from(e))
            })
            .and_then(move |(socket, _reminde_buf)| {
                length_delimited::Framed::new(socket)
                    .send(Bytes::from_static(VERSION_HEAD))
                    .map(|stream| stream)
                    .deadline(Instant::now() + Duration::from_secs(timeout_secs))
                    .map_err(|e| ::std::convert::From::from(e))
            })
            .and_then(move |stream| {
                stream
                    .into_future()
                    .map(move |(frame, stream)| {
                        let bytes = frame.unwrap();
                        let buf = bytes.as_ref();
                        let (_, (version, addr_or_time)) = decode_ack(buf).unwrap();
                        Session {
                            addr,
                            stream: stream,
                            version: version,
                            addr_or_time: addr_or_time.to_string(),
                        }
                    })
                    .map_err(|e| ::std::convert::From::from(e.0))
            })
    }

    pub fn dispatch(
        &self,
        from_tws: Sender<IncomingMessage>,
        to_tws: Receiver<OutgoingMessage>,
    ) -> impl Future<Item = Session, Error = SessionError> {
        let (to_socket, from_socket) = self.stream.split();

        from_socket
            .select(to_tws)
            .map_err(|e| ::std::convert::From::from(e))
            .map(move |self| self)
        /*let reader = from_socket
            .map_err(|e| ::std::convert::From::from(e))
            .fold(from_tws, |from_tws, msg| from_tws.send(decode(msg)));


        let writer = to_tws
            .map_err(|()| unreachable!("rx can't fail"))
            .fold(to_socket, |to_socket, msg| to_socket.send(encode(msg)));

        reader.select(writer).map_err(|e| ::std::convert::From::from(e)).map(|self| self)*/
    }

    /*pub fn runloop(
        session: Session,
        from_tws: Sender<IncomingMessage>,
        to_tws: Receiver<OutgoingMessage>,
    ) -> impl Future<Item = Session, Error = SessionError> {
        let (to_socket, from_socket) = session.stream.split();

        let reader = from_socket.for_each(move |msg| {
            let msg = decode(msg);
            from_tws.send(msg);

            //tx.send(msg)
            //.map_err(|e| SessionError::SendError(e))
            if let Err(_) = from_tws.send(msg) {
                println!("cannot send to message queue");
            }

            Ok(())
        });

        let writer = to_tws
            .map_err(|()| unreachable!("rx can't fail"))
            .fold(to_socket, |to_socket, msg| to_socket.send(encode(msg)));
        //.map(|_| ());

        // Use select to allow either the reading or writing half dropping to drop the other
        // half. The `map` and `map_err` here effectively force this drop.
        reader
            .select(writer)
            .map_err(|e| eprintln!("Read Error:"))
            .map(|_| ())
    }


    pub fn runloop2(
        session: Session,
        from_tws: Sender<IncomingMessage>,
        to_tws: Receiver<OutgoingMessage>,
    ) -> impl Future<Item = Session, Error = SessionError> {
        let (to_socket, from_socket) = session.stream.split();

        let reader = from_socket.for_each(move |msg| {
            let msg = decode(msg);
            from_tws.send(msg);

            //tx.send(msg)
            //.map_err(|e| SessionError::SendError(e))
            if let Err(_) = from_tws.send(msg) {
                println!("cannot send to message queue");
            }

            Ok(())
        });

        let writer = to_tws
            .map_err(|()| unreachable!("rx can't fail"))
            .fold(to_socket, |to_socket, msg| to_socket.send(encode(msg)));
        //.map(|_| ());

        // Use select to allow either the reading or writing half dropping to drop the other
        // half. The `map` and `map_err` here effectively force this drop.
        reader
            .select(writer)
            .map_err(|e| eprintln!("Read Error:"))
            .map(|_| ())
    }*/
}

/*fn _connect_with_timeout(
        addr: &SocketAddr,
        timeout_secs: u64,
    ) -> impl Future<Item = TcpStream, Error = SessionError> {
        //} DeadlineError<std::io::Error>> {
        TcpStream::connect(addr)
            .deadline(Instant::now() + Duration::from_secs(timeout_secs))
            .map_err(|e| SessionError::DeadlineError(e))
    }

    fn _handshake(
        stream: TcpStream,
        timeout_secs: u64,
    ) -> impl Future<Item = TcpStream, Error = SessionError> {
        tokio::io::write_all(stream, API_HEAD)
            .deadline(Instant::now() + Duration::from_secs(timeout_secs))
            .map_err(|e| SessionError::DeadlineError(e))
            .map(move |(stream, _reminde_buf)| stream)
    }

    fn _write_version_head(
        stream: TcpStream,
        timeout_secs: u64,
    ) -> impl Future<Item = Packet, Error = SessionError> {
        let framed = length_delimited::Framed::new(stream);
        framed
            .send(Bytes::from_static(VERSION_HEAD))
            .map(move |stream| stream)
            .map_err(|e| SessionError::IoError(e))
    }

    fn _read_first_packet<'a>(
        packet: Packet,
    ) -> impl Future<Item = (Packet, i32, &'a str), Error = SessionError> {
        packet
            .into_future()
            .map(move |(frame, stream)| {
                let bytes = frame.unwrap();
                let buf = bytes.as_ref();
                let (_, (version, addr_or_time)) = decode_ack(buf).unwrap();
                (stream, version, addr_or_time)
            })
            .map_err(|e| SessionError::IoError(e.0))
    }

    pub fn connect(
        addr: &SocketAddr,
        timeout_secs: u64,
    ) -> impl Future<Item = Self, Error = SessionError> {
        Session::_connect_with_timeout(addr, 1)
            .and_then(move |stream| Session::_handshake(stream, 1))
            .and_then(move |stream| Session::_write_version_head(stream, 1))
            .and_then(move |framed| Session::_read_first_packet(framed))
            .map(move |(stream, version, addr_or_time)| Session {
                stream,
                version,
                addr_or_time: addr_or_time.to_string(),
            })
}*/

/*.and_then(move |session| {
                let api = api::start_api(session.version, "".to_string()).unwrap();
                let buf = bytes::Bytes::from(api.buf.as_slice());
                session
                    .stream
                    .send(buf)
                    .map(|session| session)
                    .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Handlshake error"))
});*/

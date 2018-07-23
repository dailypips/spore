extern crate futures;
extern crate tokio;
extern crate tokio_io;

use std::net::SocketAddr;
use tokio::net::TcpStream;

use futures::Future;

struct SessionError {}
struct Session {}
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
                    //.map_err(|e| ::std::convert::From::from(e))
                    .from_err()
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
                    .into_future()  // read first handshake message
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
                    //.map_err(|e| e.0)
            })
    }
}

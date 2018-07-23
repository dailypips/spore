#[macro_use]
extern crate log;

#[macro_use]
extern crate nom;

extern crate bytes;
extern crate chrono;
extern crate failure;
extern crate futures;
extern crate pgyer_ib;
extern crate tokio;
extern crate tokio_codec;
extern crate tokio_io;

use bytes::Bytes;
use futures::Future;
use std::i32;
use std::net::SocketAddr;
use std::str::from_utf8;
//use std::time::{Duration, Instant};
use chrono::{DateTime, FixedOffset};
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio_io::codec::length_delimited;

use std::str::{from_utf8_unchecked, FromStr};

use pgyer_ib::session::Session;

fn main() {
    let port = std::env::args().nth(1).unwrap_or("".to_string());
    let port = port.parse::<u32>().unwrap_or(7496);

    let addr = format!("{}:{}", "127.0.0.1", port);
    println!("connecting .. {}", addr);
    let addr = addr.parse::<SocketAddr>().unwrap();

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let session = Session::connect(addr, 1)
        .map_err(|e| eprintln!("Read Error: {:?}", e))
        .map(|session| {
            println!("Session {:?}", session);
            ()
        });
    rt.spawn(session);

    println!("session created ok");

    rt.shutdown_on_idle().wait().unwrap();
}

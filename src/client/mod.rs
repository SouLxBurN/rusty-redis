use std::net::TcpStream;
use std::str;
use anyhow::Error;

use crate::{RedisStreamable, BUF_MAX};

pub fn connect(url: &str) -> Result<(), Error> {
    let stream = TcpStream::connect(url)?;
    do_something(stream)?;
    Ok(())
}

fn do_something(mut stream: TcpStream) -> Result<(), Error> {
    println!("Connection Established");
    stream.write_message("Hello! I'm Client.".as_bytes())?;

    let mut read_buf = [0u8; BUF_MAX];
    if let Ok(n) = stream.read_message(&mut read_buf) {
        println!("{}", str::from_utf8(&read_buf[0..n]).unwrap());
    }

    Ok(())
}

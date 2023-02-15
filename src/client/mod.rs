use std::str;
use anyhow::Error;

use crate::{BUF_MAX, RedisConnection};

pub async fn connect(url: &str) -> Result<(), Error> {
    let connection = RedisConnection::connect(url).await?;
    do_something(connection).await?;
    Ok(())
}

async fn do_something(mut stream: RedisConnection) -> Result<(), Error> {
    println!("Connection Established");
    stream.write_message("Hello! I'm Client.".as_bytes()).await?;

    let mut read_buf = [0u8; BUF_MAX];
    if let Ok(n) = stream.read_message(&mut read_buf).await {
        println!("{}", str::from_utf8(&read_buf[0..n]).unwrap());
    }

    Ok(())
}

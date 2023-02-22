use std::{str, io};
use tokio::net::TcpStream;

use crate::RedisConnection;

pub async fn connect(url: &str) -> Result<RedisConnection<TcpStream>, io::Error> {
    RedisConnection::connect(url).await
}


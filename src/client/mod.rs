use std::io::{ErrorKind, Error};
use std::{str, io};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::command::Command;
use crate::response::Response;
use crate::{RedisConnection, BUF_MAX};

pub async fn connect(url: &str) -> Result<RedisConnection<TcpStream>, io::Error> {
    RedisConnection::connect(url).await
}

impl RedisConnection<TcpStream> {
    async fn connect(url: &str) -> Result<Self, Error> {
        let stream = TcpStream::connect(url).await?;
        Ok(RedisConnection{stream})
    }
}

impl<T> RedisConnection<T>
where
    T: AsyncReadExt + AsyncWriteExt + Unpin,
{
    /// Fetch an item from cache matching the provided key.
    pub async fn get(&mut self, key: String) -> Result<Response, anyhow::Error> {
        self.write_command(Command::GET(key)).await?;
        self.read_response().await
    }

    /// Return a full list all keys present on the cache server.
    pub async fn keys(&mut self) -> Result<Response, anyhow::Error> {
        self.write_command(Command::KEYS).await?;
        self.read_response().await
    }

    // TODO: Add TLL to set command
    /// Store a key->value in the cache.
    pub async fn set(&mut self, key: String, value: Vec<u8>) -> Result<Response, anyhow::Error> {
        self.write_command(Command::SET(key, value)).await?;
        self.read_response().await
    }

    /// Remove an item from cache with the given key.
    pub async fn delete(&mut self, key: String) -> Result<Response, anyhow::Error> {
        self.write_command(Command::DELETE(key)).await?;
        self.read_response().await
    }

    async fn read_response(&mut self) -> Result<Response, anyhow::Error> {
        let mut buffer = [0u8; BUF_MAX];
        let b = self.stream.read(&mut buffer).await?;
        if b == 0 {
            return Err(io::Error::new(ErrorKind::UnexpectedEof, "Stream read 0 bytes").into())
        }
        Ok(Response::deserialize(&buffer)?)
    }

    async fn write_command(&mut self, cmd: Command) -> io::Result<()> {
        self.stream.write_all(cmd.encode().as_slice()).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test::io::Builder;

    #[tokio::test]
    async fn test_read_response() {
        let mut builder = Builder::new();
        let (mock, mut handle) = builder.build_with_handle();

        let expected_message = "Hello there!";
        let mut response: Vec<u8> = vec![];
        response.extend_from_slice(&2u32.to_le_bytes());
        response.append(&mut (expected_message.as_bytes().len() as u32).to_le_bytes().to_vec());
        response.append(&mut expected_message.as_bytes().to_vec());
        handle.read(response.as_slice());

        let mut conn = RedisConnection::new(mock);
        let n = conn
            .read_response()
            .await
            .expect("Failed to read mock buffer");

        assert!(matches!(n, Response::String(..)));
        match n {
            Response::String(s) => assert_eq!(expected_message, s),
            _ => assert!(false)
        }
    }
}

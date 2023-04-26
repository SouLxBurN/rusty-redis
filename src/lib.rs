use crate::command::Command;
use crate::response::Response;
use std::collections::VecDeque;
use std::str;
use std::sync::Arc;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, Error, ErrorKind};
use tokio::net::TcpStream;

pub mod client;
pub mod server;
pub mod response;
pub mod command;

pub const BUF_MAX: usize = 256;

pub struct RedisConnection<T>
where
    T: AsyncReadExt + AsyncWriteExt + Unpin,
{
    stream: T,
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
    fn new(stream: T) -> Self {
        RedisConnection { stream }
    }

    async fn read_command(&mut self) -> io::Result<VecDeque<Vec<u8>>> {
        let mut buffer = [0u8; BUF_MAX];
        let b = self.stream.read(&mut buffer).await?;
        if b == 0 {
            return Err(io::Error::new(ErrorKind::UnexpectedEof, "Stream read 0 bytes"))
        }

        let len_buf: &[u8; 4] = &buffer[0..4].try_into().unwrap();
        let cmd_len = u32::from_le_bytes(*len_buf) as usize;

        let mut strs = VecDeque::new();
        let mut remaining = &buffer[4..];
        for _ in 0..cmd_len {
            let str_len = u32::from_le_bytes(remaining[0..4].try_into().unwrap()) as usize;
            let st = remaining[4..str_len + 4].to_vec();
            strs.push_back(st);
            remaining = &remaining[str_len + 4..]
        }
        assert_eq!(cmd_len, strs.len());

        Ok(strs)
    }

    pub async fn read_response(&mut self) -> Result<Response, anyhow::Error> {
        let mut buffer = [0u8; BUF_MAX];
        let b = self.stream.read(&mut buffer).await?;
        if b == 0 {
            return Err(io::Error::new(ErrorKind::UnexpectedEof, "Stream read 0 bytes").into())
        }
        Ok(Response::deserialize(&buffer)?)
    }

    pub async fn write_command(&mut self, cmd: Command) -> io::Result<()> {
        self.stream.write_all(cmd.encode().as_slice()).await?;
        Ok(())
    }

    async fn write_response(&mut self, response: Response) -> io::Result<()> {
        self.stream.write_all(&response.serialize()).await?;
        Ok(())
    }

    async fn write_message(&mut self, buffer: Arc<&[u8]>) -> io::Result<()> {
        let msg_len = buffer.len() as u32;
        self.stream.write_all(&msg_len.to_le_bytes()).await?;
        self.stream.write_all(&buffer[0..buffer.len()]).await?;
        Ok(())
    }
}

// TODO: Clean up the unwraps, and handle bad command data gracefully.
pub fn parse_command(mut cmd_str: VecDeque<Vec<u8>>) -> Result<Command, anyhow::Error> {
    let cmd = cmd_str.pop_front();
    if let Some(cmd) = cmd {
        match String::from_utf8(cmd.to_vec())?.as_str() {
            "get" => Ok(Command::GET(String::from_utf8(cmd_str.pop_front().unwrap())?.to_string())),
            "keys" => Ok(Command::KEYS),
            "del" => Ok(Command::DELETE(String::from_utf8(cmd_str.pop_front().unwrap())?.to_string())),
            "set" => Ok(Command::SET(String::from_utf8(cmd_str.pop_front().unwrap())?.to_string(), cmd_str.pop_front().unwrap())),
            _s => Err(Error::new(ErrorKind::Unsupported, format!("unsupported command: {}", _s)).into()),
        }
    } else {
        Err(Error::new(ErrorKind::UnexpectedEof, "Failed to parse command").into())
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

    #[tokio::test]
    async fn test_read_command() {
        let mut builder = Builder::new();
        let (mock, mut handle) = builder.build_with_handle();

        let expected = VecDeque::from(
            [
                b"set".to_vec(),
                b"1234".to_vec(),
                b"Hello Stream!".to_vec()
            ]);
        handle.read(Command::SET("1234".to_string(), b"Hello Stream!".to_vec()).encode().as_slice());

        let mut conn = RedisConnection::new(mock);
        let actual = conn.read_command().await.expect("Failed to read commands");

        assert_eq!(expected, actual);
    }
}

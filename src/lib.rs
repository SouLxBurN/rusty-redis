use tokio::io::{AsyncWriteExt, AsyncReadExt, self, Error, ErrorKind};
use tokio::net::TcpStream;

pub mod server;
pub mod client;

pub const BUF_MAX: usize = 128;

pub struct RedisConnection {
    stream: TcpStream
}

impl RedisConnection {
    fn new(stream: TcpStream) -> Self {
        RedisConnection{stream}
    }

    async fn connect(url: &str) -> Result<Self, Error> {
        let stream = TcpStream::connect(url).await?;
        Ok(RedisConnection{stream})
    }

    async fn read_message(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let mut len_buf = [0u8; 4];
        self.stream.read_exact(&mut len_buf).await?;
        let msg_len = u32::from_le_bytes(len_buf) as usize;
        assert!(msg_len < BUF_MAX);

        let b = self.stream.read(buffer).await?;
        if b < msg_len {
            Err(Error::from(ErrorKind::UnexpectedEof))
        } else {
            Ok(msg_len)
        }
    }

    async fn write_message(&mut self, buffer: &[u8]) -> io::Result<()> {
        let msg_len = buffer.len() as u32;
        self.stream.write_all(&msg_len.to_le_bytes()).await?;
        self.stream.write_all(buffer).await?;
        Ok(())
    }
}

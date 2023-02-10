use std::io::{Write, Read, self, Error, ErrorKind};
use std::net::TcpStream;

pub mod server;
pub mod client;

pub const BUF_MAX: usize = 128;

pub trait RedisStreamable {
    fn read_message(&mut self, buffer: &mut [u8]) -> io::Result<usize>;
    fn write_message(&mut self, buffer: &[u8]) -> io::Result<()>;
}

impl RedisStreamable for TcpStream {
    fn read_message(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let mut len_buf = [0u8; 4];
        self.read_exact(&mut len_buf)?;
        let msg_len = u32::from_le_bytes(len_buf) as usize;
        assert!(msg_len < BUF_MAX);

        let b = self.read(buffer)?;
        if b < msg_len {
            Err(Error::from(ErrorKind::UnexpectedEof))
        } else {
            Ok(msg_len)
        }
    }

    fn write_message(&mut self, buffer: &[u8]) -> io::Result<()> {
        let msg_len = buffer.len() as u32;
        self.write_all(&msg_len.to_le_bytes())?;
        self.write_all(buffer)?;
        Ok(())
    }
}

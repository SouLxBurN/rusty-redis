use std::collections::VecDeque;
use std::io::ErrorKind;
use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt, self};

use crate::response::Response;
use crate::{RedisConnection, BUF_MAX};

impl<T> RedisConnection<T>
where
    T: AsyncReadExt + AsyncWriteExt + Unpin,
{
    pub async fn read_command(&mut self) -> io::Result<VecDeque<Vec<u8>>> {
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

    pub async fn write_response(&mut self, response: Response) -> io::Result<()> {
        self.stream.write_all(&response.serialize()).await?;
        Ok(())
    }

    pub async fn write_message(&mut self, buffer: Arc<&[u8]>) -> io::Result<()> {
        let msg_len = buffer.len() as u32;
        self.stream.write_all(&msg_len.to_le_bytes()).await?;
        self.stream.write_all(&buffer[0..buffer.len()]).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::command::Command;

    use super::*;
    use tokio_test::io::Builder;
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

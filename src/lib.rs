use tokio::io::{AsyncReadExt, AsyncWriteExt};

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

impl<T> RedisConnection<T>
where
    T: AsyncReadExt + AsyncWriteExt + Unpin,
{
    fn new(stream: T) -> Self {
        RedisConnection { stream }
    }
}


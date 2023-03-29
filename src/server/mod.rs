mod table;
use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use crate::{RedisConnection, parse_command};
use table::HTable;

pub struct RedisServer {
    host: String,
    port: u32,
    cache: Arc<RwLock<HTable>>
}

impl RedisServer {
    pub fn new(host: String, port: u32) -> Self {
        let cache = Arc::new(RwLock::new(HTable::new(64usize)));
        RedisServer{host, port, cache}
    }

    pub async fn start_server(&self) {
        if let Ok(listener) = TcpListener::bind(
            format!("{}:{}", self.host, self.port)).await {
            self.listen(listener).await;
        }
    }

    async fn listen(&self, listener: TcpListener) {
        loop {
            let (stream, _addr) = listener.accept().await.expect("Failed to accept connection");
            println!("Receiving Incoming Transmission");
            let cache = self.cache.clone();
            let mut conn = RedisConnection::new(stream);
            tokio::spawn(async move {
                loop {
                    if let Ok(cmd) = conn.read_command().await {
                        match parse_command(cmd) {
                            Ok(the_cmd) => {
                                match the_cmd {
                                    crate::Command::GET(key) => execute_get(&mut conn, cache.clone(), &key).await,
                                    crate::Command::KEYS() => execute_keys(&mut conn, cache.clone()).await,
                                    crate::Command::SET(key, value) => execute_set(&mut conn, cache.clone(), &key, value).await,
                                    crate::Command::DELETE(key) => execute_delete(&mut conn, cache.clone(), &key).await,
                                };
                            },
                            Err(e) => println!("invalid command received: {}", e),
                        }
                    } else {
                        println!("connection ended");
                        break;
                    }
                }
            });
        }
    }
}

async fn execute_keys<T>(conn: &mut RedisConnection<T>, cache: Arc<RwLock<HTable>>)
    where T: AsyncReadExt + AsyncWriteExt + Unpin
{
    println!("KEYS");
    let cache_read = cache.read().await;
    let keys = cache_read.keys();
    if let Err(e) = conn.write_message(keys.as_slice().join(",").as_bytes()).await {
        eprintln!("Failed to write message {}", e);
    }
}

async fn execute_get<T>(conn: &mut RedisConnection<T>, cache: Arc<RwLock<HTable>>, key: &str)
    where T: AsyncReadExt + AsyncWriteExt + Unpin
{
    println!("GET {key}");
    let cache_read = cache.read().await;
    if let Some(data) = cache_read.get(key) {
        if let Err(e) = conn.write_message(data.as_slice()).await {
            eprintln!("Failed to write message {}", e);
        }
    } else {
        if let Err(e) = conn.write_message(b"None").await {
            eprintln!("Failed to write message {}", e);
        }
    }
}

async fn execute_set<T>(conn: &mut RedisConnection<T>, cache: Arc<RwLock<HTable>>, key: &str, value: Vec<u8>)
    where T: AsyncReadExt + AsyncWriteExt + Unpin
{
    println!("SET {key}: {}", String::from_utf8(value.to_vec()).unwrap());
    let mut cache_rw = cache.write().await;
    cache_rw.insert(key, value);
    if let Err(e) = conn.write_message("Hi Client! I'm Dad!".as_bytes()).await {
        eprintln!("Failed to write message {}", e);
    }
}

async fn execute_delete<T>(conn: &mut RedisConnection<T>, cache: Arc<RwLock<HTable>>, key: &str)
    where T: AsyncReadExt + AsyncWriteExt + Unpin
{
    println!("DEL {key}");
    let mut cache_rw = cache.write().await;
    cache_rw.delete(key);
    if let Err(e) = conn.write_message("Hi Client! I'm Dad!".as_bytes()).await {
        eprintln!("Failed to write message {}", e);
    }
}

mod table;
mod tree;
mod store;
mod connection;

use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tokio::time::sleep;
use rusty_redis_core::command::Command;
use rusty_redis_core::response::Response;
use crate::connection::RedisServerConnection;

use self::store::DataStore;

pub struct RedisServer {
    host: String,
    port: u32,
    store: Arc<RwLock<DataStore>>
}

impl RedisServer {
    pub fn new(host: String, port: u32) -> Self {
        let store = Arc::new(RwLock::new(DataStore::new(64usize)));
        RedisServer{host, port, store}
    }

    pub async fn start_server(&self) {
        let data_store = self.store.clone();
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_millis(1000)).await;
                let ex_keys = {
                    let write_store = data_store.write();
                    write_store.await.expire()
                };
                if let Some(expired) = ex_keys {
                    println!("Cache Key ({:?}) expired", expired);
                };
            }
        });
        if let Ok(listener) = TcpListener::bind(
            format!("{}:{}", self.host, self.port)).await {
            self.listen(listener).await;
        }
    }

    async fn listen(&self, listener: TcpListener) {
        loop {
            let (stream, _addr) = listener.accept().await.expect("Failed to accept connection");
            println!("Receiving Incoming Transmission");
            let data_store = self.store.clone();
            let mut conn = RedisServerConnection::new(stream);
            tokio::spawn(async move {
                loop {
                    if let Ok(cmd) = conn.read_command().await {
                        match Command::parse(cmd) {
                            Ok(the_cmd) => {
                                match the_cmd {
                                    Command::GET(key) => execute_get(&mut conn, data_store.clone(), &key).await,
                                    Command::KEYS => execute_keys(&mut conn, data_store.clone()).await,
                                    Command::SET(key, value, ttl) => execute_set(&mut conn, data_store.clone(), &key, value, ttl).await,
                                    Command::DELETE(key) => execute_delete(&mut conn, data_store.clone(), &key).await,
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

async fn execute_keys<T>(conn: &mut RedisServerConnection<T>, data_store: Arc<RwLock<DataStore>>)
    where T: AsyncReadExt + AsyncWriteExt + Unpin
{
    println!("KEYS");
    let store_read = data_store.read().await;
    let keys = store_read.keys();
    let response = Response::Array(Arc::new(keys.to_vec()));
    if let Err(e) = conn.write_response(response).await {
        eprintln!("Failed to write message {}", e);
    }
}

async fn execute_get<T>(conn: &mut RedisServerConnection<T>, data_store: Arc<RwLock<DataStore>>, key: &str)
    where T: AsyncReadExt + AsyncWriteExt + Unpin
{
    println!("GET {key}");
    let store_read = data_store.read().await;
    if let Some(data) = store_read.get(key) {
        let response = Response::Data(data.clone());
        if let Err(e) = conn.write_response(response).await {
            eprintln!("Failed to write response {}", e);
        }
    } else {
        // return nil, if we had nil in Rust.
        if let Err(e) = conn.write_response(Response::Empty).await {
            eprintln!("Failed to write response {}", e);
        }
    }
}

async fn execute_set<T>(conn: &mut RedisServerConnection<T>, data_store: Arc<RwLock<DataStore>>, key: &str, value: Vec<u8>, ttl: u64)
    where T: AsyncReadExt + AsyncWriteExt + Unpin
{
    println!("SET {key}: {}", String::from_utf8(value.to_vec()).unwrap());
    let mut store_rw = data_store.write().await;
    store_rw.insert(key, value, ttl);
    if let Err(e) = conn.write_response(Response::String(String::from("Hi Client! I'm Dad!"))).await {
        eprintln!("Failed to write message {}", e);
    }
}

async fn execute_delete<T>(conn: &mut RedisServerConnection<T>, cache: Arc<RwLock<DataStore>>, key: &str)
    where T: AsyncReadExt + AsyncWriteExt + Unpin
{
    println!("DEL {key}");
    let mut cache_rw = cache.write().await;
    cache_rw.delete(key);
    if let Err(e) = conn.write_response(Response::String(String::from("Hi Client! I'm Dad!"))).await {
        eprintln!("Failed to write message {}", e);
    }
}

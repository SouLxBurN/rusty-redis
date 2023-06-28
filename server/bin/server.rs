use rusty_redis::RedisServer;

#[tokio::main]
async fn main() {
    let host = String::from("0.0.0.0");
    let port = 8081u32;
    RedisServer::new(host, port).start_server().await;
}


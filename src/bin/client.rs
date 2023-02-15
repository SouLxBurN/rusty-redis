use anyhow::Error;
use rusty_redis::client;

#[tokio::main]
async fn main() -> Result<(), Error>{
    client::connect("localhost:8080").await
}

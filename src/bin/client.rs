use anyhow::Error;
use rusty_redis::client;

fn main() -> Result<(), Error>{
    client::connect("localhost:8080")
}

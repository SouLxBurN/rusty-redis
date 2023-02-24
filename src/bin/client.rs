use std::collections::VecDeque;

use rusty_redis::{client, BUF_MAX, RedisConnection};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    if let Ok(mut conn) = client::connect("localhost:8080").await {
        println!("Connection Established");
        let _ = conn.write_command(
            VecDeque::from(
                [
                    "set".as_bytes().to_vec(),
                    "1234".as_bytes().to_vec(),
                    "{\"hello\":\"stream!\"}".as_bytes().to_vec()
                ]
            )
        ).await;
        wait_and_read_response(&mut conn).await;

        let _ = conn.write_command(
            VecDeque::from(
                [
                    "get".as_bytes().to_vec(),
                    "1234".as_bytes().to_vec()
                ]
            )
        ).await;
        wait_and_read_response(&mut conn).await;

        let _ = conn.write_command(
            VecDeque::from(
                [
                    "del".as_bytes().to_vec(),
                    "1234".as_bytes().to_vec()
                ]
            )
        ).await;
        wait_and_read_response(&mut conn).await;
    }
}

async fn wait_and_read_response(conn: &mut RedisConnection<TcpStream>) {
    let mut read_buf = [0u8; BUF_MAX];
    match conn.read_response(&mut read_buf).await {
        Ok(n) => println!("{}", std::str::from_utf8(&read_buf[0..n]).unwrap()),
        Err(_e) => {
            println!("server connection closed");
        }
    }
}

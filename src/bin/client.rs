use rusty_redis::{client, BUF_MAX};

#[tokio::main]
async fn main() {
    if let Ok(mut conn) = client::connect("localhost:8080").await {
        println!("Connection Established");
        let write_result = conn.write_command(
            vec!( "Hello!", "I'm", "Client")
        ).await;
        if write_result.is_ok() {
            loop {
                let mut read_buf = [0u8; BUF_MAX];
                match conn.read_response(&mut read_buf).await {
                    Ok(n) => println!("{}", std::str::from_utf8(&read_buf[0..n]).unwrap()),
                    Err(_e) => {
                        println!("server connection closed");
                        break;
                    }
                }
            }
        }
    }
}

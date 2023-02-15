use std::str;
use tokio::net::TcpListener;
use crate::{BUF_MAX, RedisConnection};

pub async fn start_server() {
    if let Ok(listener) = TcpListener::bind("0.0.0.0:8080").await {
        listen(listener).await;
    }
}

async fn listen(listener: TcpListener) {
    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                println!("Receiving Incoming Transmission");
                let mut conn = RedisConnection::new(stream);
                tokio::spawn(async move {
                    let mut read_buf = [0u8; BUF_MAX];
                    loop { // Maintain Connection
                        if let Ok(n) = conn.read_message(&mut read_buf).await {
                            println!("{}", str::from_utf8(&read_buf[0..n]).unwrap());
                            if let Err(e) = conn.write_message("Hi Client! I'm Dad!".as_bytes()).await {
                                eprintln!("Failed to write message {}", e);
                                break;
                            }
                        } else {
                            println!("read_full ended");
                            break;
                        }
                    }
                });
            },
            Err(e) => panic!("Failed to accept connection: {}", e),
        }
    }
}

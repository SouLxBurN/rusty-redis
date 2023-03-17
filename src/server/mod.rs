use tokio::net::TcpListener;
use crate::{RedisConnection, parse_command};

mod table;

pub async fn start_server() {
    if let Ok(listener) = TcpListener::bind("0.0.0.0:8080").await {
        listen(listener).await;
    }
}

async fn listen(listener: TcpListener) {
    loop {
        let (stream, _addr) = listener.accept().await.expect("Failed to accept connection");
        println!("Receiving Incoming Transmission");
        let mut conn = RedisConnection::new(stream);
        tokio::spawn(async move {
            loop {
                if let Ok(cmd) = conn.read_command().await {
                    match parse_command(cmd) {
                        Ok(the_cmd) => {
                            match the_cmd {
                                crate::Command::GET(key) => println!("GET {key}"),
                                crate::Command::SET(key, value) => println!("SET {key}: {}", String::from_utf8(value.to_vec()).unwrap()),
                                crate::Command::DELETE(key) => println!("DEL {key}"),
                            }

                            if let Err(e) = conn.write_message("Hi Client! I'm Dad!".as_bytes()).await {
                                eprintln!("Failed to write message {}", e);
                                break;
                            }
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

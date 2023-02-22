use tokio::net::TcpListener;
use crate::RedisConnection;

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
                    cmd.iter().for_each(|c| {
                        print!("{c} ");
                    });
                    print!("\n");

                    if let Err(e) = conn.write_message("Hi Client! I'm Dad!".as_bytes()).await {
                        eprintln!("Failed to write message {}", e);
                        break;
                    }
                } else {
                    println!("connection ended");
                    break;
                }
            }
        });
    }
}

use std::{net::TcpListener, str};
use crate::{BUF_MAX, RedisStreamable};

pub fn start_server() {
    if let Ok(listener) = TcpListener::bind("0.0.0.0:8080") {
        listen(listener);
    }
}

fn listen(listener: TcpListener) {
    loop {
        match listener.accept() {
            Ok((mut stream, _addr)) => {
                println!("Receiving Incoming Transmission");
                loop { // Maintain Connection
                    let mut full_buf = [0u8; BUF_MAX];
                    if let Ok(n) = stream.read_message(&mut full_buf) {
                        println!("{}", str::from_utf8(&full_buf[0..n]).unwrap());
                        if let Err(e) = stream.write_message("Hi Client! I'm Dad!".as_bytes()) {
                            eprintln!("Failed to write message {}", e);
                            break;
                        }
                    } else {
                        println!("read_full ended");
                        break;
                    }
                }
            },
            Err(e) => panic!("Failed to accept connection: {}", e),
        }
    }
}

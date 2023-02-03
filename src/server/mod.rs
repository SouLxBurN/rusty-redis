use std::io::{Write, Read};
use std::net::{TcpListener, TcpStream};
use std::str;

// fd = socket()
// bind(fd, address)
// listen(fd)
// while True:
//     conn_fd = accept(fd)
//     do_something_with(conn_fd)
//     close(conn_fd)

pub fn start_server() {
    if let Ok(listener) = TcpListener::bind("0.0.0.0:8080") {
        listen(listener);
    }
}

fn listen(listener: TcpListener) {
    loop {
        match listener.accept() {
            Ok((stream, _addr)) => {
                do_something(stream);
            },
            Err(e) => panic!("Failed to accept connection: {}", e),
        }
    }
}

fn do_something(mut stream: TcpStream) {
    println!("Connection Accepted");

    let mut buffer = [0u8; 8];
    while let Ok(n) = stream.read(&mut buffer) {
        print!("{}", str::from_utf8(&buffer[0..n]).unwrap());
        if n < 8 {
            print!("\n");
            break;
        }
    }

    let msg = "Hello There!";
    stream.write(msg.as_bytes()).unwrap();
}

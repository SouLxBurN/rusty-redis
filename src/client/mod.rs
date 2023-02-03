use std::io::{Write, Read};
use std::net::TcpStream;
use std::str;

// fd = socket()
// connect(fd, address)
// do_something_with(fd)
// close(fd)

pub fn connect() {
    if let Ok(stream) = TcpStream::connect("localhost:8080") {
        do_something(stream);
    }
}

fn do_something(mut stream: TcpStream) {
    println!("Connection Established");

    let msg = "Hello! My name is Client.";
    stream.write(msg.as_bytes()).unwrap();

    let mut buffer = [0u8; 8];
    while let Ok(n) = stream.read(&mut buffer) {
        print!("{}", str::from_utf8(&buffer[0..n]).unwrap());
        if n < 8 {
            print!("\n");
            break;
        }
    }

}

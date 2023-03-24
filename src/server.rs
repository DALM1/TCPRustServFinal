use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buf;
    loop {
        buf = [0; 512];
        match stream.read(&mut buf) {
            Ok(n) => {
                if n == 0 {
                    return;
                }
                let message = String::from_utf8_lossy(&buf[..n]);
                println!("{}", message);
            }
            Err(_) => {
                return;
            }
        }
    }
}

pub fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:5000")?;
    println!("Server listening on port 5000");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    Ok(())
}
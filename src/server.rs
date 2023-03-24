use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = [0; 512];
    let mut username = String::new();

    // Read username from the client
    match stream.read(&mut buf) {
        Ok(n) => {
            username = String::from_utf8_lossy(&buf[..n]).to_string();
            println!("{} has joined the chat", username);
        }
        Err(e) => {
            println!("Error reading from client: {}", e);
            return Err(e);
        }
    }

    loop {
        let mut buf = [0; 512];
        match stream.read(&mut buf) {
            Ok(n) => {
                if n == 0 {
                    println!("{} has left the chat", username);
                    return Ok(());
                }
                let message = String::from_utf8_lossy(&buf[..n]).to_string();
                println!("{}: {}", username, message);
            }
            Err(e) => {
                println!("Error reading from client: {}", e);
                return Err(e);
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:5000")?;
    println!("Server started on port 5000");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread for each client connection
                thread::spawn(move || {
                    if let Err(e) = handle_client(stream) {
                        println!("Error handling client: {}", e);
                    }
                });
            }
            Err(e) => {
                println!("Error accepting client connection: {}", e);
            }
        }
    }

    Ok(())
}

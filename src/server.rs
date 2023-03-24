use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream, clients: &mut Vec<TcpStream>) {
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

                // Send the message to all clients
                for client in clients.iter_mut() {
                    if client.peer_addr().unwrap() != stream.peer_addr().unwrap() {
                        client.write_all(message.as_bytes()).unwrap();
                    }
                }
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

    let mut clients = Vec::new();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let peer_addr = stream.peer_addr().unwrap();
                println!("New connection from {}", peer_addr);
                clients.push(stream.try_clone()?);
                thread::spawn(|| {
                    handle_client(stream, &mut clients);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    Ok(())
}

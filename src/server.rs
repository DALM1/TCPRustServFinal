use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

fn listen() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:5000")?;
    println!("Server listening on port 5000");

    let clients: Arc<Mutex<Vec<(String, TcpStream)>>> = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        let stream = stream.expect("Failed to establish connection");

        let clients = Arc::clone(&clients);
        let mut stream_clone = stream.try_clone().expect("Failed to clone stream");

        thread::spawn(move || {
            let username = get_username(&mut stream_clone).expect("Failed to get username");
            let message = format!("{} has joined the chat", username);
            broadcast_message(&clients, &message, None);

            clients.lock().unwrap().push((username.clone(), stream_clone));

            loop {
                let message = read_message(&mut stream_clone).expect("Failed to read message");
                let message = format!("{}: {}", username, message);
                broadcast_message(&clients, &message, Some(&username));
            }

            let message = format!("{} has left the chat", username);
            broadcast_message(&clients, &message, None);
            clients.lock().unwrap().retain(|client| client.0 != username);
        });
    }

    Ok(())
}

fn read_message(stream: &mut TcpStream) -> std::io::Result<String> {
    let mut reader = BufReader::new(stream);
    let mut message = String::new();
    reader.read_line(&mut message)?;
    Ok(message.trim().to_string())
}

fn get_username(stream: &mut TcpStream) -> std::io::Result<String> {
    let mut reader = BufReader::new(stream);
    let mut username = String::new();
    write!(stream, "Enter your username: ")?;
    reader.read_line(&mut username)?;
    Ok(username.trim().to_string())
}

fn broadcast_message(clients: &Arc<Mutex<Vec<(String, TcpStream)>>>, message: &str, sender: Option<&String>) {
    let clients = clients.lock().unwrap();
    for client in clients.iter() {
        if let Some(sender) = sender {
            if client.0 == *sender {
                continue;
            }
        }
        let mut client_clone = client.1.try_clone().expect("Failed to clone client");
        writeln!(client_clone, "{}", message).expect("Failed to send message to client");
    }
}

fn main() -> std::io::Result<()> {
    listen()?;
    Ok(())
}

use std::io::{self, Write};
use std::net::TcpStream;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

fn read_input(input_tx: Sender<String>) -> io::Result<()> {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input_tx.send(input.trim().to_string())?;
    }
}

fn read_stream(stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) -> io::Result<()> {
    let mut reader = io::BufReader::new(&stream);
    loop {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let mut clients = clients.lock().unwrap();
        for client in &mut *clients {
            if client.peer_addr().unwrap() != stream.peer_addr().unwrap() {
                client.write_all(line.as_bytes())?;
            }
        }
    }
}

fn write_stream(
    stream: TcpStream,
    clients: Arc<Mutex<Vec<TcpStream>>>,
    output_rx: Receiver<String>,
) -> io::Result<()> {
    loop {
        let output = output_rx.recv().unwrap();
        let mut clients = clients.lock().unwrap();
        for client in &mut *clients {
            if client.peer_addr().unwrap() != stream.peer_addr().unwrap() {
                client.write_all(output.as_bytes())?;
            }
        }
    }
}

fn main() -> io::Result<()> {
    let listener = std::net::TcpListener::bind("127.0.0.1:5000")?;

    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        let clients = Arc::clone(&clients);

        let mut stream = stream?;

        let clients_clone = Arc::clone(&clients);
        clients.lock().unwrap().push(stream.try_clone()?);

        let (input_tx, input_rx) = std::sync::mpsc::channel();
        let (output_tx, output_rx) = std::sync::mpsc::channel();

        thread::spawn(move || read_stream(stream, clients_clone));
        thread::spawn(move || write_stream(stream, clients_clone, output_rx));
        thread::spawn(move || read_input(input_tx));

        loop {
            let input = input_rx.recv().unwrap();
            if input == ":quit" {
                break;
            }
            output_tx.send(format!("{}\n", input))?;
        }

        let mut clients = clients.lock().unwrap();
        clients.retain(|client| client.peer_addr().unwrap() != stream.peer_addr().unwrap());
    }

    Ok(())
}

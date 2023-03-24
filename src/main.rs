use std::io::{self, BufRead, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;

const LOCAL: &str = "127.0.0.1:5000";
const MSG_SIZE: usize = 32;

fn main() {
    let mut client = TcpStream::connect(LOCAL).expect("Failed to connect to server.");

    println!("Enter your username:");
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read input.");
    let username = username.trim();

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buf = vec![0; MSG_SIZE];
        match client.read_exact(&mut buf) {
            Ok(_) => {
                let msg = buf.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                println!("{}", String::from_utf8_lossy(&msg));
            }
            Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Connection with server was severed.");
                break;
            }
        }
        match rx.try_recv() {
            Ok(msg) => {
                let mut buf = msg.clone().into_bytes();
                buf.resize(MSG_SIZE, 0);
                client.write_all(&buf).expect("Failed to send message.");
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }
        thread::sleep(std::time::Duration::from_millis(100));
    });

    loop {
        let mut msg = String::new();
        io::stdin()
            .read_line(&mut msg)
            .expect("Failed to read input.");
        let msg = msg.trim().to_owned();
        if msg == ":quit" || tx.send(msg).is_err() {
            break;
        }
    }
    println!("Bye bye!");
}

use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

pub fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:5000")?;
    println!("Connected to server on port 5000");

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        stream.write(input.as_bytes())?;

        let mut buf = [0; 512];
        match stream.read(&mut buf) {
            Ok(n) => {
                if n == 0 {
                    return Ok(());
                }
                let message = str::from_utf8(&buf[..n]).unwrap();
                println!("{}", message);
            }
            Err(e) => {
                println!("Error: {}", e);
                return Err(e);
            }
        }
    }
}

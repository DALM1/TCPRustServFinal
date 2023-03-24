mod server;
mod client;

fn main() {
    std::thread::spawn(|| {
        server::main().unwrap();
    });

    client::main().unwrap();
}
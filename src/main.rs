mod server;

fn main() {
    std::thread::spawn(|| {
        server::main().unwrap();
    });

    client::main().unwrap();
}
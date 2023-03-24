mod server;

fn main() {
    std::thread::spawn(|| {
        server::main().unwrap();
    });

    loop {}
}

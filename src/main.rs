mod server;
use server::*;
use std::thread;

fn main() {
    loop {
        let server = Server::new("127.0.0.1:8080");
        let thread = thread::spawn(move || {
            server.run();
        });
        let _answer = thread.join().unwrap();
    }
}

mod server;
use server::*;
use std::thread;

fn main() {
    loop {
        let server = Server::new("192.168.0.4:8080");
        let thread = thread::spawn(move || {
            server.run();
        });
        let _answer = thread.join().unwrap();
    }
}

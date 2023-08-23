use super::multithread::ThreadPool;
use super::request::Request;
use std::net::TcpListener;

#[derive(Debug)]
pub struct Server {
    ip: &'static str,
    port: &'static str,
}

impl Server {
    pub fn new(s: &'static str) -> Self {
        let (ip, port) = s.split_once(":").unwrap();
        return Self { ip, port };
    }

    pub fn run(self) {
        let listner = match TcpListener::bind(self.ip.to_string() + ":" + self.port) {
            Ok(listner) => listner,
            Err(_) => panic!("can`t to listning by this address"),
        };
        let pool = ThreadPool::new(4);

        for stream in listner.incoming().take(2) {
            let stream = stream.unwrap();
            pool.execute(|| {
                Request::request_handler(stream);
            })
        }
    }
}

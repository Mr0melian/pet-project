//Server idea in thetings that you give previous when you initialaise your server with addr, ports, root foulder and ather singth.
//for this we need to make same filds in ouer struct.
//i can`t do this in solo
//for example we can put in server object ip addr same ports(http, sql, etc.) also we can put some anauther data like folders or root url.




use super::blogic::Blogic;
use super::multithread::ThreadPool;
use super::request::{ Request, self};
use std::net::TcpListener;
use std::io::prelude::*;

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
            let logic: Vec<Blogic> = Vec::new();
            let mut stream = stream.unwrap();
            let request = pool.execute(move || {
                let request = Request::request_handler(&stream);
                println!("{}\n{}\n{:?}\n{}\n", request.http, request.url, request.headers, request.body );
                let response = Blogic::blogic_aparat(logic, request);
                stream.write(format!("{} {}{}", response.http, response.status_code, response.body).as_bytes());
                
            });

            
            //stream.write(format!("{} {}{}", response.http, response.status_code, response.body).as_bytes());
        }
    }
}

use std::collections::HashMap;
use std::io::Read;
use std::net::TcpStream;
use super::utils::{headers_spliter, spliter};

pub struct Request {
    pub method: Method,
    pub url: String,
    pub http: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Request {
    pub fn new(request: String) -> Result<Self, RequestError> {
        let massege = spliter(request, "\r\n\r\n");
        let mut lines = spliter(massege.get(0).unwrap().to_string(), "\r\n");

        let s = spliter(lines.remove(0).to_string(), " ");

        let method = Method::method_handler(&s[0])?;

        let (url, http) = (s.get(1).unwrap().to_string(), s.get(2).unwrap().to_string());

        let headers = headers_spliter(lines);

        let body = massege.get(1).unwrap().to_string();
        

        return Ok(Request {
            method,
            url,
            http,
            headers,
            body,
        });
    }

    pub fn request_handler<'a>(mut stream: &TcpStream) -> Request {
        let mut buff: [u8; 1024] = [0; 1024];
        stream.read(&mut buff).expect("faild to read");
        let request = String::from_utf8_lossy(&buff[..]).to_string();
        println!("{}", request);
        return Request::new(request).unwrap();
    }




}

#[derive(Debug)]
pub enum Method {
    GET,
    PUT,
    SUBMIT,
    POST,
    CONNECT,
    DELETE,
    HEAD,
    OPTIONS,
    TRACE,
}
impl Method {
    fn method_handler(s: &str) -> Result<Method, RequestError> {
        match s.to_string().to_uppercase().trim() {
            "GET" => Ok(Method::GET),
            "PUT" => Ok(Method::PUT),
            "SUBMIT" => Ok(Method::SUBMIT),
            "POST" => Ok(Method::POST),
            "CONNECT" => Ok(Method::CONNECT),
            "DELETE" => Ok(Method::DELETE),
            "HEAD" => Ok(Method::HEAD),
            "OPTIONS" => Ok(Method::OPTIONS),
            "TRACE" => Ok(Method::TRACE),
            _ => Err(RequestError::InvalidMethod(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub enum RequestError {
    InvalidMethod(String),
}

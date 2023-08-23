use std::collections::HashMap;
use std::io::Read;
use std::net::TcpStream;
use std::str::from_utf8;
use utils::headers_spliter;

use super::{spliter, utils};

#[derive(Debug)]

pub struct Request {
    method: Method,
    url: String,
    http: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl Request {
    pub fn new(request: &str) -> Result<Self, RequestError> {
        let mut lines = spliter(request, "\r\n");

        let s = spliter(lines.remove(0), " ");

        let method = Method::method_handler(s[0])?;

        let (url, http) = (s[1].to_string(), s[2].to_string());

        let headers = headers_spliter(lines.clone());

        let body = Option::None;

        return Ok(Request {
            method,
            url,
            http,
            headers,
            body,
        });
    }
    pub fn request_handler(mut stream: TcpStream) -> Request {
        let mut buff: [u8; 1024] = [0; 1024];
        stream.read(&mut buff).expect("faild to read");
        let request = from_utf8(&buff).unwrap();
        println!("{}", request);
        return Request::new(request).unwrap();
    }
}

#[derive(Debug)]
enum Method {
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

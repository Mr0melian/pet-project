use super::request::Request;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Response {
    pub http: String,
    pub status_code: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Response {
    pub fn new(request: Request, body: String) -> Self {
        let http = request.http;
        let headers = request.headers;
        let status_code = "200 OK\r\n\r\n".to_string();
        Self {
            http,
            status_code,
            headers,
            body,
        }
    }

    pub fn new404(request:  Request, body: String) -> Self {
        let http = request.http;
        let headers = request.headers;
        let status_code = "404 NOT FOUND\r\n\r\n".to_string();
        Self {
            http,
            status_code,
            headers,
            body,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Url(&'static str);

pub mod blogic {
    use super::Request;
    use super::Response;
    use super::Url;
    use std::fs;


    pub struct Blogic {
        url: Url,
        func: Func,
    }

    impl Blogic {
        pub fn new<F>(url: Url, f: F) -> Self
        where
            F: FnOnce(&Request) -> String + Send + 'static,
        {
            Self {
                url,
                func: Box::new(f),
            }
        }

        fn find_by_url(logic: Vec<Self>, url: &str) -> Option<Self> {
            for func in logic {
                if func.url.0 == url {
                    return Some(func);
                }
            }
            return None;
        }

        fn err_404() -> String {
            let body = fs::read_to_string("../pet-project/source/404.html").expect("can`t to reed");

            body
        }

        pub fn blogic_aparat(logic: Vec<Self>, request:  Request) -> Response {
            let logic = if let Some(logic) = Self::find_by_url(logic, &request.url) {
                logic.func
            } else {
                let body = Self::err_404();
                return Response::new404(request, body);
            };
            let body = logic(&request);
            Response::new(request, body)
        }
    }

    type Func = Box<dyn FnOnce(&Request) -> String + Send + 'static>;    
}

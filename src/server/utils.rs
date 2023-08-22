use std::collections::HashMap;

pub fn spliter<'a>(mut massage: &'a str, option: &str) -> Vec<&'a str> {
    let mut v: Vec<&str> = Vec::new();
    loop {
        let chars = match massage.find(option) {
            Some(char) => char,
            None => {
                break;
            }
        };
        v.push(&massage[..chars]);
        massage = &massage[chars + 2..];
    }
    v.push(massage);
    return v;
}

pub fn headers_spliter(s: Vec<&str>) -> HashMap<String, String> {
    let mut headers: HashMap<String, String> = HashMap::new();
    for str in s {
        let (key, value): (&str, &str) = match str.split_once(':') {
            Some((key, value)) => (key.trim(), value.trim()),
            None => {
                break;
            }
        };
        headers.insert(key.to_string(), value.to_string());
    }
    return headers;
}

pub fn slise<T>(mut v: Vec<T>, num: usize) -> Vec<T> {
    v.remove(num);
    v
}

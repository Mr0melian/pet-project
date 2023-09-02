use std::collections::HashMap;

pub fn spliter(mut massage:  String, option: &str) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    loop{
        if let Some((line, mass)) = massage.split_once(option){
            v.push(line.to_string());
            massage = mass.to_string();
        }else{
            break;
        }
    }
    v.push(massage);
    v
}

pub fn headers_spliter(s: Vec<String>) -> HashMap<String, String> {
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

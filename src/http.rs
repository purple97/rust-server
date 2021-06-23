use json::JsonValue;
use regex::Regex;
use std::fs::read_to_string;
use std::io::{Read, Result, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::str::from_utf8;

fn string_to_json(jsonstr: &str) -> (&str, &str, &str) {
    let mut new_str: Vec<&str> = jsonstr.split(": ").collect();
    return (
        jsonstr,
        &new_str.get_mut(0).unwrap().trim(),
        &new_str.get_mut(1).unwrap().trim(),
    );
}

fn query_client_context(request: &String) -> JsonValue {
    let mut query: Vec<&str> = request.lines().filter(|x| x.len() > 0).collect();
    let re = Regex::new(r"^(.*):").unwrap();
    let mut data = get_client_match(&query.get_mut(0).unwrap().to_string());
    for value in &query {
        if re.is_match(value) {
            let (_m, k, v) = string_to_json(value);
            data[k] = v.into();
        }
    }
    data
}

fn get_client_match(request: &String) -> JsonValue {
    let re = Regex::new(r"^(.+) (.*) (.+)/([1-2]\.[0-9]*)").unwrap();
    let mut data = JsonValue::new_object();
    for t in re.captures_iter(request) {
        data["type"] = t[1].into();
        data["pathname"] = t[2].into();
        data["protocol"] = t[3].into();
        data["protocolVersion"] = t[4].into();
    }
    data
}

/*  */
fn set_content(_status_code: u8, content: String) -> String {
    let status = "200 OK";
    let body = format!("HTTP/1.1 {}\r\n\r\n{}", status, content);
    body
}
/*  */
fn render(filename: &Path) -> String {
    if filename.is_file() {
        let contents = read_to_string(filename).unwrap();
        return contents;
    } else {
        println!("No such file or directory :{}", filename.display());
        return "<body><h1>No such file or directory</h1></body>".to_string();
    }
}

/*  */
pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024 * 8];
    // let mut buffer = [0; 512];
    let filename = Path::new("index.html");
    let contents = render(&filename);
    // 读取请求体内容
    stream.read(&mut buffer).unwrap();
    let context = from_utf8(&buffer).unwrap().to_string();
    let data = query_client_context(&context);
    println!("请求类型：{}", data["type"]);
    /* Response 内容 */
    let response = set_content(200, contents);
    // 写入内容
    stream.write(response.as_bytes()).unwrap();
    //刷新socket
    stream.flush().unwrap();
}

/*
 */
pub fn create_server() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3333")?;
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

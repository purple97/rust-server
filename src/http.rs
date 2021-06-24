use json::JsonValue;
use regex::Regex;
use std::fs::read_to_string;
use std::io::{Read, Result, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::str::from_utf8;

/*
kev:value字符串拆解
@return (json_string, key, value);
 */
fn string_to_json(json_str: &str) -> (&str, &str, &str) {
    let mut new_str: Vec<&str> = json_str.split(": ").collect();
    return (
        json_str,
        &new_str.get_mut(0).unwrap().trim(),
        &new_str.get_mut(1).unwrap().trim(),
    );
}

/*
请求体拆解成json数据
*/
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

/*
*/
fn get_client_match(request: &String) -> JsonValue {
    let re = Regex::new(r"^(.+) (.*) (.+)/([0-9]\.[0-9]*)").unwrap();
    let mut data = JsonValue::new_object();
    for t in re.captures_iter(request) {
        data["method"] = t[1].into();
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

/*
读取文件并返回内容;
*/
fn render(filename: &Path) -> String {
    if filename.is_file() {
        let contents = read_to_string(filename).unwrap();
        return contents;
    } else {
        println!("No such file or directory :{}", filename.display());
        return "<body><h1>No such file or directory</h1></body>".to_string();
    }
}

/*
链接句柄
*/
pub fn handle_client(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0u8; 1024 * 8];
    // let mut buffer = [0; 512];
    let filename = Path::new("index.html");
    // 读取请求体内容
    stream.read(&mut buffer)?;
    let context = from_utf8(&buffer).unwrap().to_string();
    let data = query_client_context(&context);
    println!("请求类型：{}", data["method"]);
    /* Response 内容 */
    let contents = render(&filename);
    let response = set_content(200, contents);
    // 写入内容
    stream.write(response.as_bytes())?;
    //刷新socket
    stream.flush()?;
    Ok(())
}

/*
创建服务
 */
pub fn create_server(port: u32) -> Result<()> {
    let host = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(host)?;
    for stream in listener.incoming() {
        return handle_client(stream?);
    }
    Ok(())
}

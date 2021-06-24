# rust 练习 - 服务器

## 随手记

-   网络请求的本质是个 socket 链接

### 监听请求、端口

-   引入模块

```rust
use std::net::{TcpListener, TcpStream};
```

-   监听一个地址和端口:

```Rust
let listener = TcpListener::bind(host)?; //这个问号是 .unwrap()的语法糖, 需要返回 Result<()> 才可以使用该语法糖
for stream in listener.incoming() {
    handle_client(stream?);
}
```

-   看到 stream 就知道， 请求是流的形式过来

```rust
// 定义一个变量存储整个流
let mut buffer = [0;1024];
// 读取流内容到变量
stream.read(&mut buffer).unwrap();
//给流写入内容
stream.write("<body><h1>Hello world</h1></body>".as_bytes()).unwrap();
```

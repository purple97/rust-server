mod http;
use http::create_server;

fn main() {
    create_server(3333).unwrap()
}

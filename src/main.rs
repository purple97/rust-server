mod http;
use http::create_server;

fn main() -> std::io::Result<()> {
    create_server(3333)?;
    Ok(())
}

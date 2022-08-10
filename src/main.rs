use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    zero2prod::run(TcpListener::bind("127.0.0.1:0")?)?.await
}

use std::net::TcpListener;

use zero2prod::configuration::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("failed to read configuration");
    let address = format!("127.0.0.1:{}", config.application_port);

    zero2prod::run(TcpListener::bind(&address)?)?.await
}

use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::configuration::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("failed to read configuration");
    let address = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(&address)?;

    let connection_pool = PgPool::connect_lazy(&config.database.connection_string())
        .expect("Failed to connect to postgres");

    zero2prod::run(connection_pool, listener)?.await
}

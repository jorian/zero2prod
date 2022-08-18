use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use tracing::info;
use zero2prod::configuration::get_configuration;
use zero2prod::telemetry::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    info!("tracing initiated");

    let config = get_configuration().expect("failed to read configuration");
    let address = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(&address)?;

    let connection_pool = PgPoolOptions::new().connect_lazy_with(config.database.with_db());

    zero2prod::run(connection_pool, listener)?.await
}

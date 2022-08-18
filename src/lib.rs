pub mod configuration;
pub mod routes;
pub mod startup;
pub mod telemetry;

use actix_web::{dev::Server, web, App, HttpServer};
use routes::*;
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(connection_pool: PgPool, listener: TcpListener) -> Result<Server, std::io::Error> {
    let web_data = web::Data::new(connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("health_check", web::get().to(health_check::health_check))
            .route(
                "subscriptions",
                web::post().to(subscriptions::subscribe),
            )
            .app_data(web_data.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

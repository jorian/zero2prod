pub mod configuration;
pub mod routes;
pub mod startup;

use actix_web::{dev::Server, web, App, HttpServer};
use routes::*;
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("health_check", web::get().to(health_check::health_check))
            .route("subscriptions", web::post().to(subscriptions::subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

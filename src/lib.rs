use std::net::Ipv4Addr;

use actix_web::{middleware::Logger, services, App, HttpServer};
mod routes;

pub async fn run(ip: Ipv4Addr, port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(services![routes::index])
    })
    .bind((ip, port))?
    .run()
    .await
}

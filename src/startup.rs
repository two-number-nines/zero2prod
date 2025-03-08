use crate::routes;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use routes::health_check::*;
use routes::subscriptions::*;
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

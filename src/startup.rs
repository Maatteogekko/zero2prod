use crate::routes::{health_check, subscribe};
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::{io, net::TcpListener};

pub fn run(listener: TcpListener, db_pool: PgPool) -> io::Result<Server> {
    let connection = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health-check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

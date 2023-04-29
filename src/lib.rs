use std::{io, net::TcpListener};

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};

pub fn run(listener: TcpListener) -> io::Result<Server> {
    let server = HttpServer::new(|| App::new().route("/health-check", web::get().to(health_check)))
        .listen(listener)?
        .run();

    Ok(server)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

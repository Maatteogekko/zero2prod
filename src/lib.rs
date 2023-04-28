use std::io;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

pub async fn run() -> io::Result<()> {
    HttpServer::new(|| App::new().route("/health-check", web::get().to(health_check)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

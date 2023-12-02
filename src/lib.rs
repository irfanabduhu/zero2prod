use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    // HttpServer handles all transport-level concerns: listening port,
    // number of concurrent connections, TLS etc.
    let server = HttpServer::new(|| {
        // App handles incoming requests and serves responses.
        // It deals with application logic: routing, middlewares, request handlers etc.
        App::new().route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

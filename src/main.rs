use zero2prod::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = std::net::TcpListener::bind("127.0.0.1:8000").expect("Failed to bind port 8000");
    run(listener)?.await
}

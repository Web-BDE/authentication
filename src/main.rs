use actix_web::{web, App, HttpResponse, HttpServer};

mod authentication;
mod authorization;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(HttpResponse::Ok))
            .service(authentication::service::authenticate)
            .service(authorization::service::authorize)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

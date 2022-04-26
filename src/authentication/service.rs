use actix_web::{post, HttpResponse, Responder};

#[post("/authenticate")]
pub async fn authenticate() -> impl Responder {
    HttpResponse::Ok().body("Authenticated")
}
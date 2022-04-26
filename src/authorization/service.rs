use actix_web::{post, HttpResponse, Responder};

#[post("/authorize")]
pub async fn authorize() -> impl Responder {
    HttpResponse::Ok().body("Authorized")
}

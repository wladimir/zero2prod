use actix_web::{HttpResponse, web};

#[derive(serde::Deserialize)]
pub struct Payload {
    name: String,
    email: String,
}

pub async fn subscribe(_payload: web::Json<Payload>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
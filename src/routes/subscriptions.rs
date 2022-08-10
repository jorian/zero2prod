use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(_form_data: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}

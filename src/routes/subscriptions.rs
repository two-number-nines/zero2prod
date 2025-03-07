use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    dbg!(&_form.name);
    dbg!(&_form.email);
    HttpResponse::Ok().finish()
}
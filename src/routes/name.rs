use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct Name {
    first_name: String,
    last_name: String,
}

#[post("/name")]
pub async fn full_name(name: web::Json<Name>) -> impl Responder {
    let full_name = format!("{} {}", name.first_name, name.last_name);
    let greeting = format!("Hello {}", full_name);

    HttpResponse::Ok().json(json!({
        "greeting": greeting
    }))
}

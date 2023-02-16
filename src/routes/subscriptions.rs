use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

// In Rust, by default, items defined within a module are private and cannot be accessed from outside the module.
// If you want to make an item publicly accessible,
// you need to use the pub keyword in front of the item definition:

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

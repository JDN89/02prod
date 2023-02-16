
use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}


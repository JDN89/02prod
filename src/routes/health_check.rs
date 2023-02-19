use actix_web::{App, HttpResponse, HttpServer};

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}


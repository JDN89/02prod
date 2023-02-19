//! src/lib.rs
use std::net::TcpListener;

use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::dev::Server;
use sqlx::PgPool;

use crate::routes::{health_check, subscribe};

// Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword
// We have no .await call, so it is not needed anymore.
pub fn run(listener: TcpListener,
           db_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap the connection in a smart pointer
    let db_pool = web::Data::new(db_pool);
    // Capture `connection` from the surrounding environment
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))

            .app_data(db_pool.clone())

    })
        .listen(listener)?
        .run();
    Ok(server)
}

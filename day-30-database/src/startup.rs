// use crate::routes::{health_check, subscribe};
// use actix_web::dev::Server;
// use actix_web::{App, HttpServer, web};
// use std::net::TcpListener;

// pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
//     let server = HttpServer::new(|| {
//         App::new()
//             .route("/health_check", web::get().to(health_check))
//             .route("/subscriptions", web::post().to(subscribe))
//     })
//     .listen(listener)?
//     .run();
//     Ok(server)
// }

// ⛳️ Step 8 - The Data Extractor
use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
// use sqlx::PgConnection;
use std::net::TcpListener;

// pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {
//     // Wrap the connection in a smart pointer
//     let connection = web::Data::new(connection);
//     // Capture `connection` from the surrounding environment
//     let server = HttpServer::new(move || {
//         App::new()
//             .route("/health_check", web::get().to(health_check))
//             .route("/subscriptions", web::post().to(subscribe))
//             // Get a pointer copy and attach it to the application state
//             .app_data(connection.clone())
//     })
//     .listen(listener)?
//     .run();
//     Ok(server)
// }

use sqlx::PgPool;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap the pool using web::Data, which boils down to an Arc smart pointer
    let db_pool = web::Data::new(db_pool);
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

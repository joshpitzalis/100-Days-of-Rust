use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap the pool using web::Data, which boils down to an Arc smart pointer
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            // Middlewares are added using the `wrap` method on `App`
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

// // ⛳️ Step 5 - Ensure all logs for a particular request have a Request Id

// use crate::routes::{health_check, subscribe};
// use actix_web::dev::Server;
// use actix_web::web::Data;
// use actix_web::{App, HttpServer, web};
// use sqlx::PgPool;
// use std::net::TcpListener;
// use tracing_actix_web::TracingLogger;

// pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
//     let db_pool = Data::new(db_pool);
//     let server = HttpServer::new(move || {
//         App::new()
//             // Instead of `Logger::default`
//             .wrap(TracingLogger::default())
//             .route("/health_check", web::get().to(health_check))
//             .route("/subscriptions", web::post().to(subscribe))
//             .app_data(db_pool.clone())
//     })
//     .listen(listener)?
//     .run();
//     Ok(server)
// }

// // TEST_LOG=true cargo test subscribe_returns_a_200_for_valid_form_data | bunyan
// // Now you should see a request_id on all logs as well as request_path and a few other useful bits of information.

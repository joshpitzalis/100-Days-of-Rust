use actix_web::dev::Server;
use actix_web::{App, HttpResponse, HttpServer, web};
use std::net::TcpListener;

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// Now update `run()` with the final handler:
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

// // ⛳️ Step 4 - Writing Our First Query

// pub mod configuration;
// pub mod routes;
// pub mod startup;

// pub use startup::run;

// // We want to migrate the file structure to:
// // src/
// //  configuration.rs
// //  lib.rs
// //  main.rs
// //  routes/
// //      mod.rs
// //      health_check.rs
// //      subscriptions.rs
// //  startup.rs

// // Run cargo test
// // Make sure cargo test comes out green before moving forward.
// //   ✅ test health_check_works ... ok
// //   ✅ test subscribe_returns_a_200_for_valid_form_data ... ok
// //   ✅ test subscribe_returns_a_400_when_data_is_missing ... ok

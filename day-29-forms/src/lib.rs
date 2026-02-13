// ⛳️ Step 1 - Write integration tests for the subscription endpoint

// go to tests/health_check.rs

use actix_web::dev::Server;
use actix_web::{App, HttpResponse, HttpServer, web};
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .listen(listener)?
        .run();
    Ok(server)
}

// // ⛳️ Step 2 - Add the POST /subscriptions route
// // Register a new route in the App for POST requests to /subscriptions.
// // For now, the handler always returns 200 OK regardless of input.

// use actix_web::dev::Server;
// use actix_web::{App, HttpResponse, HttpServer, web};
// use std::net::TcpListener;

// async fn subscribe() -> HttpResponse {
//     HttpResponse::Ok().finish()
// }

// async fn health_check() -> HttpResponse {
//     HttpResponse::Ok().finish()
// }

// // Then update `run()` to register the new route:
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

// // Run: cargo test
// // This will make the "valid form data" test pass, but the
// // "missing data" test will still fail (it expects a 400).

// // ⛳️ Step 3 - Define the FormData struct with serde deserialization
// // To parse the form body, we need a struct that matches the expected fields.
// // `#[derive(serde::Deserialize)]` auto-generates the Deserialize implementation,
// // which tells serde how to build this struct from serialized data.
// //
// // Run: cargo add serde --features derive
// //
// // serde itself is format-agnostic — it defines a data model and traits
// // (Serialize/Deserialize). Concrete formats like JSON or URL-encoding
// // are handled by separate crates (serde_json, serde_urlencoded, etc.).
// // actix-web uses `serde_urlencoded` under the hood for Form extraction.

// #[derive(serde::Deserialize)]
// struct FormData {
//     email: String,
//     name: String,
// }

// // ⛳️ Step 4 - Use the Form extractor to parse and validate input
// // actix-web's `web::Form<T>` is an *extractor*: a type that implements the `FromRequest` trait. When used as a handler argument, actix-web
// // automatically calls `Form::from_request()` before your handler runs.
// //
// // Here's what happens under the hood:
// //   1. actix-web sees `web::Form<FormData>` in the handler signature
// //   2. It calls `Form::from_request(req, payload)`
// //   3. Inside, `serde_urlencoded::from_bytes::<FormData>(&body)` is called
// //   4. serde uses the derived `Deserialize` impl to parse the URL-encoded body
// //   5. If parsing succeeds → handler runs, you get a `Form<FormData>`
// //   6. If parsing fails (missing fields, wrong types) → 400 BAD REQUEST is
// //      returned automatically, handler is never called
// //
// // This is the power of actix-web extractors: strongly-typed input
// // with automatic validation — no manual parsing needed.
// use actix_web::dev::Server;
// use actix_web::{App, HttpResponse, HttpServer, web};
// use std::net::TcpListener;

// #[derive(serde::Deserialize)]
// struct FormData {
//     email: String,
//     name: String,
// }

// async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
//     HttpResponse::Ok().finish()
// }

// async fn health_check() -> HttpResponse {
//     HttpResponse::Ok().finish()
// }

// // Now update `run()` with the final handler:
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

// // Run: cargo test

// // All 3 tests should now pass:
// //   ✅ health_check_works
// //   ✅ subscribe_returns_a_200_for_valid_form_data
// //   ✅ subscribe_returns_a_400_when_data_is_missing

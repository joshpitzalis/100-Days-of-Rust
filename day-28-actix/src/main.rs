fn main() {
    println!("Hello, world!");
}

// // ⛳️ Step 1 - Wire up Actix-Web with a basic greeting handler
// // Actix-Web is one of Rust's oldest and most battle-tested async web frameworks.
// // It runs on Tokio, Rust's most popular async runtime.
// // Run: cargo add actix-web@4
// // Run: cargo add tokio@1 --features macros,rt-multi-thread
// //
// // We import the core building blocks:
// // - `web`: helpers for routing (e.g., `web::get()` creates a GET route guard)
// // - `App`: the application builder where all routing and middleware lives
// // - `HttpRequest`: the incoming request object with path, headers, query params, etc.
// // - `HttpServer`: the transport layer — handles TCP connections, TLS, concurrency
// // - `Responder`: a trait for anything that can become an HTTP response
// use actix_web::{App, HttpRequest, HttpServer, Responder, web};

// // // A handler is an async function that takes inputs and returns something
// // // that implements `Responder`. Strings implement `Responder` out of the box,
// // // so returning a `format!()` string just works.
// // // `req.match_info()` gives access to dynamic path segments like `{name}`.
// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }

// // // `#[tokio::main]` is a procedural macro that sets up Tokio's async runtime.
// // // Under the hood it wraps your async body in `tokio::runtime::Builder::new_multi_thread()`
// // // and calls `.block_on(body)` — because Rust's `main` can't actually be async.
// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     // // `HttpServer::new()` takes a closure (called once per worker thread)
//     // // that builds an `App`. The App uses the builder pattern — chain
//     // // `.route()` calls to register endpoints.
//     // // Each `.route()` takes a path and a `Route` (method guard + handler).
//     HttpServer::new(|| {
//         App::new()
//             .route("/", web::get().to(greet))
//             .route("/{name}", web::get().to(greet))
//     })
//     // // `.bind()` tells the server which address:port to listen on.
//     // // The `?` operator propagates errors (e.g., port already in use).
//     .bind("127.0.0.1:8000")?
//     .run()
//     .await
// }

// // cargo run
// // goto: http://127.0.0.1:8000/
// // goto: http://127.0.0.1:8000/<your_name>

// // ⛳️ Step 2 - Replace the greeter with a health check endpoint
// // A health check endpoint returns 200 OK with no body.
// // It's used by monitoring tools (Pingdom, Kubernetes, Nomad) to verify
// // your app is alive and responsive.
// //
// // Notice we dropped `HttpRequest` from the handler's signature entirely.
// // Actix-Web's type magic accepts a broad range of handler signatures —
// // if you don't need the request data, you can just omit it.
// //
// // `HttpResponse::Ok()` returns a builder primed with status 200.
// // Calling `.finish()` produces a response with an empty body.
// // (The builder itself also implements `Responder`, so `.finish()` is optional.)
// use actix_web::{App, HttpResponse, HttpServer, web};

// async fn health_check() -> HttpResponse {
//     HttpResponse::Ok().finish()
// }

// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
//         .bind("127.0.0.1:8000")?
//         .run()
//         .await
// }

// // cargo run
// // goto: http://127.0.0.1:8000/health_check (check netwrok tab for 200 response)

// // ⛳️ Step 3 - Restructure for testability (library + binary split)
// // Integration tests in Rust live under `tests/` and compile as separate binaries.
// // They can only import *library* crates, not binaries.
// // So we split our project: logic goes in `lib.rs`, `main.rs` is just an entrypoint.

// use day_28_actix::run;
// use std::net::TcpListener;

// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind port");
//     run(listener)?.await
// }

// // // cargo run
// // // goto: http://127.0.0.1:8000/health_check (check netwrok tab for 200 response)

// // ⛳️ Step 4 - Write a black-box integration test
// // We test the API exactly how a real client would: send HTTP requests and
// // check responses. This is fully framework-agnostic — if you rewrote the
// // server in another language, the same test logic would still apply.
// // Run: cargo add reqwest --dev
// //
// // The integration test us in tests/health_check.rs

// use day_28_actix::run;
// use std::net::TcpListener;

// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind port");
//     run(listener)?.await
// }

// // cargo test
// // you should get -> running 1 test > test health_check_works ... ok

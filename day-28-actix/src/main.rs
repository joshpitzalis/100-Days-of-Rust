// Actix-Web is one of Rust's most popular async web frameworks.
// It's built on top of Actix's actor system and Tokio for async I/O.
// Run: cargo add actix-web
//
// We start by importing the essentials:
// - `get`: An attribute macro that registers a handler for GET requests
// - `App`: The application builder where you register routes and middleware
// - `HttpServer`: The actual HTTP server that listens for connections
// - `Responder`: A trait for types that can be converted into an HTTP response
// - `HttpResponse`: Lets you build HTTP responses with status codes and bodies
use actix_web::{App, HttpResponse, HttpServer, Responder, get};

// // `#[get("/")]` is an attribute macro that does two things:
// // 1. Marks this function as a route handler
// // 2. Binds it to GET requests at the "/" path
// // The handler must be `async` because Actix-Web is fully asynchronous.
// // `impl Responder` means we can return any type that implements the Responder trait.
#[get("/")]
async fn hello() -> impl Responder {
    // // `HttpResponse::Ok()` creates a 200 status response.
    // // `.body(...)` sets the response body as plain text.
    HttpResponse::Ok().body("Hello, Rust Web!")
}

// // `#[actix_web::main]` is a macro that sets up the Tokio async runtime.
// // Without it, you'd need to manually configure the async executor.
// // It transforms `async fn main()` into a synchronous main that runs the async code.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Starting server at http://127.0.0.1:8080");

    // // `HttpServer::new()` takes a closure (a factory) that builds the App.
    // // This closure is called once per worker thread — Actix-Web spawns
    // // multiple workers to handle requests concurrently.
    HttpServer::new(|| {
        // // `App::new()` creates a fresh application instance.
        // // `.service(hello)` registers our `#[get("/")]` handler.
        // // You can chain multiple `.service()` calls to add more routes.
        App::new().service(hello)
    })
    // // `.bind(("127.0.0.1", 8080))` tells the server which address and port to listen on.
    // // The `?` operator propagates any error (e.g., port already in use).
    .bind(("127.0.0.1", 8080))?
    // // `.run()` starts the server, and `.await` keeps it running
    // // until it receives a shutdown signal (like Ctrl+C).
    .run()
    .await
}

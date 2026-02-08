// ⛳️ Step 3 - Restructure for testability (library + binary split)
//
// We refactor `run()` to return a `Server` without awaiting it.
// This lets tests spawn the server as a background task with `tokio::spawn`.
//
// Finally, we accept a `TcpListener` instead of a hardcoded address.
// This lets tests bind port 0 (OS picks a random free port) and then
// read back the actual port via `listener.local_addr().unwrap().port()`.
use actix_web::dev::Server;
use actix_web::{App, HttpResponse, HttpServer, web};
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// // `run` is no longer async — there's no `.await` inside.
// // It returns `Server` so the caller decides when to await (or spawn) it.
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        // // `.listen()` accepts an already-bound TcpListener
        // // instead of `.bind()` which takes a string address.
        .listen(listener)?
        .run();
    Ok(server)
}

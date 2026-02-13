// ⛳️ Step 1 - Write integration tests for the subscription endpoint

// go to tests/health_check.rs

use day_29_forms::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind port");
    run(listener)?.await
}

// ⛳️ Step 2 - Add the POST /subscriptions route

// go to src/lib.rs

// ⛳️ Step 3 - Define the FormData struct with serde deserialization

// go to src/lib.rs

//  ⛳️ Step 4 - Use the Form extractor to parse and validate input

//  go to src/lib.rs

use std::net::TcpListener;

// The test spawns the app on a random port, then uses `reqwest` to hit it.
// `tokio::spawn` runs the server as a background task that gets dropped
// when the test's Tokio runtime shuts down — no cleanup needed.
fn spawn_app() -> String {
    // // Port 0 tells the OS: "give me any available port."
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    // // `tokio::spawn` hands the future to the runtime for concurrent polling.
    // // The server runs in the background while our test logic executes.
    let server = day_28_actix::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

// // `#[tokio::test]` is the test equivalent of `#[tokio::main]`.
// // It spins up a fresh Tokio runtime per test and auto-adds `#[test]`.
#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert — check the full contract:
    // // 1. Status is 200 OK
    assert!(response.status().is_success());
    // // 2. Body is empty (content-length: 0)
    assert_eq!(Some(0), response.content_length());
}

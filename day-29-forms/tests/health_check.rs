// ⛳️ Step 1 - Write integration tests for the subscription endpoint
// Before writing any handler code, we capture our requirements as tests.
// We need two test cases:
//   1. A valid form (name + email) should return 200 OK
//   2. Missing fields should return 400 BAD REQUEST
//
// The test uses `reqwest` to send real HTTP requests to our server.
// Run: cargo add reqwest --dev && cargo add actix-web@4 tokio -F tokio/macros -F tokio/rt-multi-thread

use std::net::TcpListener;

fn spawn_app() -> String {
    // // Port 0 tells the OS: "give me any available port."
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    // // `tokio::spawn` hands the future to the runtime for concurrent polling.
    // // The server runs in the background while our test logic executes.
    let server = day_29_forms::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    // `application/x-www-form-urlencoded` encodes data as key=value&key=value
    // Non-alphanumeric characters are percent-encoded (space → %20, @ → %40)
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    // This is a table-driven (parametrised) test.
    // Each tuple is (invalid_body, error_message).
    // We test multiple bad inputs in one test function.
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

// Run: cargo test
// At this point, both tests will fail with 404 — we haven't added the route yet.

// ⛳️ Step 2 - Add the POST /subscriptions route

// go to src/lib.rs

// ⛳️ Step 3 - Define the FormData struct with serde deserialization

// go to src/lib.rs

//  ⛳️ Step 4 - Use the Form extractor to parse and validate input

//  go to src/lib.rs

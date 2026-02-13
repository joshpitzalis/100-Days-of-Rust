use std::net::TcpListener;

fn spawn_app() -> String {
    // // Port 0 tells the OS: "give me any available port."
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    // // `tokio::spawn` hands the future to the runtime for concurrent polling.
    // // The server runs in the background while our test logic executes.
    let server = day_30_database::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

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

// // ⛳️ Step 6 - Connecting To Postgres

// use day_30_database::configuration::get_configuration;
// use sqlx::{Connection, PgConnection};
// use std::net::TcpListener;

// fn spawn_app() -> String {
//     let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
//     let port = listener.local_addr().unwrap().port();
//     let server = day_30_database::run(listener).expect("Failed to bind address");
//     let _ = tokio::spawn(server);
//     format!("http://127.0.0.1:{}", port)
// }

// #[tokio::test]
// async fn subscribe_returns_a_200_for_valid_form_data() {
//     // Arrange
//     let app_address = spawn_app();
//     let configuration = get_configuration().expect("Failed to read configuration");
//     let connection_string = configuration.database.connection_string();
//     // The `Connection` trait MUST be in scope for us to invoke
//     // `PgConnection::connect` - it is not an inherent method of the struct!
//     let connection = PgConnection::connect(&connection_string)
//         .await
//         .expect("Failed to connect to Postgres.");
//     let client = reqwest::Client::new();
//     // Act
//     let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
//     let response = client
//         .post(&format!("{}/subscriptions", &app_address))
//         .header("Content-Type", "application/x-www-form-urlencoded")
//         .body(body)
//         .send()
//         .await
//         .expect("Failed to execute request.");
//     // Assert
//     assert_eq!(200, response.status().as_u16());
// }

// // Cargo test works  ✅
// // We can successfully connect to Postgres from our tests.
// // test subscribe_returns_a_200_for_valid_form_data ... ok

// // ⛳️ Step 7 - Making Our Test Assertion

// use day_30_database::configuration::get_configuration;
// use sqlx::{Connection, PgConnection};
// use std::net::TcpListener;

// fn spawn_app() -> String {
//     let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
//     let port = listener.local_addr().unwrap().port();
//     let server = day_30_database::run(listener).expect("Failed to bind address");
//     let _ = tokio::spawn(server);
//     format!("http://127.0.0.1:{}", port)
// }

// #[tokio::test]
// async fn subscribe_returns_a_200_for_valid_form_data() {
//     // Arrange
//     let app_address = spawn_app();
//     let configuration = get_configuration().expect("Failed to read configuration");
//     let connection_string = configuration.database.connection_string();
//     // The `Connection` trait MUST be in scope for us to invoke
//     // `PgConnection::connect` - it is not an inherent method of the struct!
//     let mut connection = PgConnection::connect(&connection_string)
//         .await
//         .expect("Failed to connect to Postgres.");
//     let client = reqwest::Client::new();
//     // Act
//     let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
//     let response = client
//         .post(&format!("{}/subscriptions", &app_address))
//         .header("Content-Type", "application/x-www-form-urlencoded")
//         .body(body)
//         .send()
//         .await
//         .expect("Failed to execute request.");
//     // Assert
//     assert_eq!(200, response.status().as_u16());

//     let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
//         .fetch_one(&mut connection)
//         .await
//         .expect("Failed to fetch saved subscription.");

//     assert_eq!(saved.email, "ursula_le_guin@gmail.com");
//     assert_eq!(saved.name, "le guin");
// }

// Cargo test FAILED  ✅
// This is exactly what we want at this stage.
// test result: FAILED. 0 passed; 1 failed;

// // ⛳️ Step 9 - Updating Our Tests

// use day_30_database::configuration::get_configuration;
// use day_30_database::startup::run;
// // use sqlx::{Connection, PgConnection};
// use sqlx::PgPool;
// use std::net::TcpListener;

// pub struct TestApp {
//     pub address: String,
//     pub db_pool: PgPool,
// }

// async fn spawn_app() -> TestApp {
//     let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
//     let port = listener.local_addr().unwrap().port();
//     let address = format!("http://127.0.0.1:{}", port);
//     let configuration = get_configuration().expect("Failed to read configuration.");
//     let connection_pool = PgPool::connect(&configuration.database.connection_string())
//         .await
//         .expect("Failed to connect to Postgres.");
//     let server = run(listener, connection_pool.clone()).expect("Failed to bind address");
//     let _ = tokio::spawn(server);
//     TestApp {
//         address,
//         db_pool: connection_pool,
//     }
// }

// #[tokio::test]
// async fn subscribe_returns_a_200_for_valid_form_data() {
//     // Arrange
//     let app = spawn_app().await;
//     let client = reqwest::Client::new();
//     // Act
//     let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
//     let response = client
//         .post(&format!("{}/subscriptions", &app.address))
//         .header("Content-Type", "application/x-www-form-urlencoded")
//         .body(body)
//         .send()
//         .await
//         .expect("Failed to execute request.");
//     // Assert
//     assert_eq!(200, response.status().as_u16());
//     let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
//         .fetch_one(&app.db_pool)
//         .await
//         .expect("Failed to fetch saved subscription.");
//     assert_eq!(saved.email, "ursula_le_guin@gmail.com");
//     assert_eq!(saved.name, "le guin");
// }

// // Cargo test works  ✅
// // test subscribe_returns_a_200_for_valid_form_data ... ok
// // And if you check your Postgres clinet you should now see the databse has a new entry!

// // ⛳️ Step 10 - Idempotency

// use day_30_database::configuration::{DatabaseSettings, get_configuration};
// use day_30_database::startup::run;
// use sqlx::{Connection, Executor, PgConnection, PgPool};
// use std::net::TcpListener;
// use uuid::Uuid;

// pub struct TestApp {
//     pub address: String,
//     pub db_pool: PgPool,
// }

// async fn spawn_app() -> TestApp {
//     let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
//     let port = listener.local_addr().unwrap().port();
//     let address = format!("http://127.0.0.1:{}", port);

//     let mut configuration = get_configuration().expect("Failed to read configuration.");
//     configuration.database.database_name = Uuid::new_v4().to_string();
//     let connection_pool = configure_database(&configuration.database).await;

//     let server = run(listener, connection_pool.clone()).expect("Failed to bind address");
//     let _ = tokio::spawn(server);
//     TestApp {
//         address,
//         db_pool: connection_pool,
//     }
// }

// pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
//     // Create database
//     let maintenance_settings = DatabaseSettings {
//         database_name: "postgres".to_string(),
//         username: "postgres".to_string(),
//         password: "password".to_string(),
//         ..config.clone()
//     };
//     let mut connection = PgConnection::connect(&maintenance_settings.connection_string())
//         .await
//         .expect("Failed to connect to Postgres");
//     connection
//         .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
//         .await
//         .expect("Failed to create database.");
//     // Migrate database
//     let connection_pool = PgPool::connect(&config.connection_string())
//         .await
//         .expect("Failed to connect to Postgres.");
//     sqlx::migrate!("./migrations")
//         .run(&connection_pool)
//         .await
//         .expect("Failed to migrate the database");
//     connection_pool
// }

// #[tokio::test]
// async fn subscribe_returns_a_200_for_valid_form_data() {
//     // Arrange
//     let app = spawn_app().await;
//     let client = reqwest::Client::new();
//     // Act
//     let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
//     let response = client
//         .post(&format!("{}/subscriptions", &app.address))
//         .header("Content-Type", "application/x-www-form-urlencoded")
//         .body(body)
//         .send()
//         .await
//         .expect("Failed to execute request.");
//     // Assert
//     assert_eq!(200, response.status().as_u16());
//     let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
//         .fetch_one(&app.db_pool)
//         .await
//         .expect("Failed to fetch saved subscription.");
//     assert_eq!(saved.email, "ursula_le_guin@gmail.com");
//     assert_eq!(saved.name, "le guin");
// }

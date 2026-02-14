// use day_31_telemetry::configuration::{DatabaseSettings, get_configuration};
// use day_31_telemetry::startup::run;
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

// ⛳️ Step 7 - Logs For Integration Tests
use day_31_telemetry::configuration::{DatabaseSettings, get_configuration};
use day_31_telemetry::startup::run;
// 🦀
use day_31_telemetry::telemetry::{get_subscriber, init_subscriber};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
// 🦀
use std::sync::LazyLock;
use uuid::Uuid;

// Ensure that the `tracing` stack is only initialised once using `LazyLock`
static TRACING: LazyLock<()> = LazyLock::new(|| {
    let subscriber = get_subscriber("test".into(), "debug".into());
    init_subscriber(subscriber);
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    // 🦀
    // The first time `initialize` is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution.
    LazyLock::force(&TRACING);
    let subscriber = get_subscriber("test".into(), "debug".into());
    init_subscriber(subscriber);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;

    let server = run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let maintenance_settings = DatabaseSettings {
        database_name: "postgres".to_string(),
        username: "postgres".to_string(),
        password: "password".to_string(),
        ..config.clone()
    };
    let mut connection = PgConnection::connect(&maintenance_settings.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");
    // Migrate database
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

// cargo test
// now you get...
// running 1 test
// {"v":0,"name":"test","msg":"sqlx::query","level":20,"hostname":"Joshs-MacBook-Pro.local","pid":41223,"time":"2026-02-14T05:58:24.777903Z","target":"sqlx::query","line":143,"file":"/Users/jxsh/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sqlx-core-0.8.6/src/logger.rs","summary":"CREATE DATABASE \"da4b9636-7e9e-4141-9f40-2279f39d7030\";","elapsed":"25.941416ms","elapsed_secs":0.025941416,"rows_affected":0,"db.statement":"","rows_returned":0}

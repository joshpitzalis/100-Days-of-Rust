use day_31_telemetry::configuration::{DatabaseSettings, get_configuration};
use day_31_telemetry::startup::run;
use day_31_telemetry::telemetry::{get_subscriber, init_subscriber};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use std::sync::LazyLock;
use uuid::Uuid;

// 🦀
static TRACING: LazyLock<()> = LazyLock::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    // We cannot assign the output of `get_subscriber` to a variable based on the
    // value TEST_LOG` because the sink is part of the type returned by
    // `get_subscriber`, therefore they are not the same type. We could work around
    // it, but this is the most straight-forward way of moving forward.
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

// static TRACING: LazyLock<()> = LazyLock::new(|| {
//     let subscriber = get_subscriber("test".into(), "debug".into());
//     init_subscriber(subscriber);
// });

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    LazyLock::force(&TRACING);
    // let subscriber = get_subscriber("test".into(), "debug".into());
    // init_subscriber(subscriber);

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

// Run:
// TEST_LOG=true cargo test subscribe_returns_a_200_for_valid_form_data | bunyan
// and you can see all the logs coming out of a certain test case to debug it

// // ⛳️ Step 4 - Secrets

// use day_31_telemetry::configuration::{DatabaseSettings, get_configuration};
// use day_31_telemetry::startup::run;
// use day_31_telemetry::telemetry::{get_subscriber, init_subscriber};
// // 🦀
// use secrecy::ExposeSecret;
// use secrecy::Secret;
// use sqlx::{Connection, Executor, PgConnection, PgPool};
// use std::net::TcpListener;
// use std::sync::LazyLock;
// use uuid::Uuid;

// static TRACING: LazyLock<()> = LazyLock::new(|| {
//     let default_filter_level = "info".to_string();
//     let subscriber_name = "test".to_string();
//     // We cannot assign the output of `get_subscriber` to a variable based on the
//     // value TEST_LOG` because the sink is part of the type returned by
//     // `get_subscriber`, therefore they are not the same type. We could work around
//     // it, but this is the most straight-forward way of moving forward.
//     if std::env::var("TEST_LOG").is_ok() {
//         let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
//         init_subscriber(subscriber);
//     } else {
//         let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
//         init_subscriber(subscriber);
//     };
// });

// pub struct TestApp {
//     pub address: String,
//     pub db_pool: PgPool,
// }

// async fn spawn_app() -> TestApp {
//     LazyLock::force(&TRACING);
//     // let subscriber = get_subscriber("test".into(), "debug".into());
//     // init_subscriber(subscriber);

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
//         // 🦀
//         password: Secret::new("password".to_string()),
//         ..config.clone()
//     };
//     // 🦀
//     let mut connection =
//         PgConnection::connect(&maintenance_settings.connection_string().expose_secret())
//             .await
//             .expect("Failed to connect to Postgres");
//     connection
//         .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
//         .await
//         .expect("Failed to create database.");
//     // 🦀
//     let connection_pool = PgPool::connect(&config.connection_string().expose_secret())
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

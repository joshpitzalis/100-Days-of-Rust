use day_31_telemetry::configuration::get_configuration;
use day_31_telemetry::startup::run;
use day_31_telemetry::telemetry::{get_subscriber, init_subscriber};
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("day_31_telemetry".into(), "info".into());
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}

// // ⛳️ Step 1 - Logs For Integration Tests
// // Refactored our initialisation logic
// // -> === tests/health_check.rs ===

// use day_31_telemetry::configuration::get_configuration;
// use day_31_telemetry::startup::run;
// use day_31_telemetry::telemetry::{get_subscriber, init_subscriber};
// use sqlx::PgPool;
// use std::net::TcpListener;

// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     // 🦀
//     let subscriber = get_subscriber("day_31_telemetry".into(), "info".into(), std::io::stdout);
//     init_subscriber(subscriber);

//     let configuration = get_configuration().expect("Failed to read configuration.");
//     let connection_pool = PgPool::connect(&configuration.database.connection_string())
//         .await
//         .expect("Failed to connect to Postgres.");
//     let address = format!("127.0.0.1:{}", configuration.application_port);
//     let listener = TcpListener::bind(address)?;
//     run(listener, connection_pool)?.await
// }

// ⛳️ Step 2 - Cleaning Up Instrumentation Code
// -> === src/routes/subscriptions.rs ===

// ⛳️ Step 3 - Better separation of concerns
// -> === src/routes/subscriptions.rs ===

// // ⛳️ Step 4 - Secrets
// //
// // [dependencies]
// // secrecy = { version= "0.8", features = ["serde"] }
// //
// // -> === src/configuration.rs ===
// // -> === tests/health_check.rs ===

// use day_31_telemetry::configuration::get_configuration;
// use day_31_telemetry::startup::run;
// use day_31_telemetry::telemetry::{get_subscriber, init_subscriber};
// // 🦀
// use secrecy::ExposeSecret;
// use sqlx::PgPool;
// use std::net::TcpListener;

// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     // 🦀
//     let subscriber = get_subscriber("day_31_telemetry".into(), "info".into(), std::io::stdout);
//     init_subscriber(subscriber);

//     let configuration = get_configuration().expect("Failed to read configuration.");
//     let connection_pool =
//         PgPool::connect(&configuration.database.connection_string().expose_secret())
//             .await
//             .expect("Failed to connect to Postgres.");
//     let address = format!("127.0.0.1:{}", configuration.application_port);
//     let listener = TcpListener::bind(address)?;
//     run(listener, connection_pool)?.await
// }

// // ⛳️ Step 5 - Ensure all logs for a particular request have a Request Id
// //
// // [dependencies]
// // tracing-actix-web= "0.7"
// //
// // -> === src/startup.rs ===
// // -> === src/routes/subscriptions.rs ===

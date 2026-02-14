// use day_31_telemetry::configuration::get_configuration;
// use day_31_telemetry::startup::run;
// use sqlx::PgPool;
// use std::net::TcpListener;

// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     let configuration = get_configuration().expect("Failed to read configuration.");
//     // Renamed!
//     let connection_pool = PgPool::connect(&configuration.database.connection_string())
//         .await
//         .expect("Failed to connect to Postgres.");
//     let address = format!("127.0.0.1:{}", configuration.application_port);
//     let listener = TcpListener::bind(address)?;
//     run(listener, connection_pool)?.await
// }

// // ⛳️ Step 1 - Instrumenting logs

// // [dependencies]
// // env_logger= "0.9"

// use day_31_telemetry::configuration::get_configuration;
// use day_31_telemetry::startup::run;
// use env_logger::Env;
// use sqlx::PgPool;
// use std::net::TcpListener;

// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     // `init` does call `set_logger`, so this is all we need to do.
//     // We are falling back to printing all logs at info-level or above
//     // if the RUST_LOG environment variable has not been set.
//     env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
//     let configuration = get_configuration().expect("Failed to read configuration.");
//     // Renamed!
//     let connection_pool = PgPool::connect(&configuration.database.connection_string())
//         .await
//         .expect("Failed to connect to Postgres.");
//     let address = format!("127.0.0.1:{}", configuration.application_port);
//     let listener = TcpListener::bind(address)?;
//     run(listener, connection_pool)?.await
// }

// // Go to
// // === startup.rs ===

// // cargo run and you shoudl see the following logs...

// // ✅ [2026-02-14T04:00:33Z INFO  actix_server::builder] starting 12 workers
// // ✅ [2026-02-14T04:00:33Z INFO  actix_server::server] Tokio runtime found; starting in existing Tokio runtime
// // ✅ [2026-02-14T04:00:33Z INFO  actix_server::server] starting service: "actix-web-service-127.0.0.1:8000", workers: 12, listening on: 127.0.0.1:8000

// // and if you go to 'http://127.0.0.1:8000/health_check' in your browser then you should also see...

// // ✅ [2026-02-14T04:00:36Z INFO  actix_web::middleware::logger] 127.0.0.1 "GET /health_check HTTP/1.1" 200 0 "-" "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36" 0.000317

// ⛳️ Step 2 - Instrumenting POST /subscriptions
// -> === src/routes/subscriptions.rs ===

// ⛳️ Step 3 - Migrating From log To tracing
// -> === src/routes/subscriptions.rs ===

// // ⛳️ Step 4 - Instrumenting Futures
// // [dependencies]
// // tracing-subscriber= { version= "0.3", features = ["registry", "env-filter"] }
// // tracing-bunyan-formatter= "0.3"
// //
// // -> === src/routes/subscriptions.rs ===

// use day_31_telemetry::configuration::get_configuration;
// use day_31_telemetry::startup::run;
// use sqlx::PgPool;
// use std::net::TcpListener;

// use tracing::subscriber::set_global_default;
// use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
// use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};

// // • tracing_subscriber::filter::EnvFilter discards spans based on their log levels and their origins

// // • tracing_bunyan_formatter::JsonStorageLayer processes spans data and stores the associated metadata in an easy-to-consume JSON format for downstream layers.

// // • tracing_bunyan_formatter::BunyanFormatterLayer builds on top of JsonStorageLayer and
// // outputs log records in bunyan-compatible JSON format.

// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     // 🦀 We removed the `env_logger` line we had before!
//     // We are falling back to printing all spans at info-level or above
//     // if the RUST_LOG environment variable has not been set.
//     let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
//     let formatting_layer = BunyanFormattingLayer::new(
//         "day_31_telemetry".into(),
//         // Output the formatted spans to stdout.
//         std::io::stdout,
//     );
//     // The `with` method is provided by `SubscriberExt`, an extension
//     // trait for `Subscriber` exposed by `tracing_subscriber`
//     let subscriber = Registry::default()
//         .with(env_filter)
//         .with(JsonStorageLayer)
//         .with(formatting_layer);
//     // `set_global_default` can be used by applications to specify
//     // what subscriber should be used to process spans.
//     set_global_default(subscriber).expect("Failed to set subscriber");
//     // 🦀 ###

//     let configuration = get_configuration().expect("Failed to read configuration.");
//     // Renamed!
//     let connection_pool = PgPool::connect(&configuration.database.connection_string())
//         .await
//         .expect("Failed to connect to Postgres.");
//     let address = format!("127.0.0.1:{}", configuration.application_port);
//     let listener = TcpListener::bind(address)?;
//     run(listener, connection_pool)?.await
// }

// // cargo run
// // and now you get...
// // {
// //     "v":0,
// //     "name":"day_31_telemetry",
// //     "msg":"starting 12 workers",
// //     "level":30,
// //     "hostname":"Jxxxl",
// //     "pid":18341,
// //     "time":"2026-02-14T05:36:18.918349Z",
// //     "target":"actix_server::builder",
// //     "line":310,
// //     "file":"/Users/jxsh/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/actix-server-2.6.0/src/builder.rs"
// // }
// // ...

// // ⛳️ Step 5 - Instrumenting Futures
// // [dependencies]
// // tracing-log= "0.2"
// //
// // tracing’s log feature flag ensures that a log record is emitted every time a tracing event happens, allowing log’s loggers to pick them up.
// // The opposite does not hold true: log does not emit tracing events out of the box and does not provide a feature flag to enable this behaviour.

// use day_31_telemetry::configuration::get_configuration;
// use day_31_telemetry::startup::run;
// use sqlx::PgPool;
// use std::net::TcpListener;
// use tracing::subscriber::set_global_default;
// use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
// // 🦀
// use tracing_log::LogTracer;
// use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};

// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     // 🦀 - Redirect all `log`'s events to our subscriber
//     LogTracer::init().expect("Failed to set logger");

//     let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
//     let formatting_layer = BunyanFormattingLayer::new("day_31_telemetry".into(), std::io::stdout);
//     let subscriber = Registry::default()
//         .with(env_filter)
//         .with(JsonStorageLayer)
//         .with(formatting_layer);
//     set_global_default(subscriber).expect("Failed to set subscriber");

//     let configuration = get_configuration().expect("Failed to read configuration.");
//     let connection_pool = PgPool::connect(&configuration.database.connection_string())
//         .await
//         .expect("Failed to connect to Postgres.");
//     let address = format!("127.0.0.1:{}", configuration.application_port);
//     let listener = TcpListener::bind(address)?;
//     run(listener, connection_pool)?.await
// }

// // All actix-web’s logs should once again be available in our console.

// // Now you can also remove all unused dependancies and cut down build times.
// // cargo install cargo-udeps

// // cargo-udeps requires the nightly compiler.
// // We add +nightly to our cargo invocation to tell cargo explicitly what toolchain we want to use.
// // cargo +nightly udeps

// ⛳️ Step 6 - Refactor

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

// -> === src/lib.rs ===
// -> === src/telemetry.rs ===

// ⛳️ Step 7 - Logs For Integration Tests
// -> === tests/health_check.rs ===

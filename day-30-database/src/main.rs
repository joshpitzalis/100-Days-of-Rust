use day_30_database::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind port");
    run(listener)?.await
}

// Run: cargo test

// All 3 tests should now pass:
//   ✅ test health_check_works ... ok
//   ✅ test subscribe_returns_a_200_for_valid_form_data ... ok
//   ✅ test subscribe_returns_a_400_when_data_is_missing ... ok

// ⛳️ Step 1 - Create a database

// cargo install --version="~0.8" sqlx-cli \
// --no-default-features \
// --features rustls,postgres

// then copy in
// === scripts/init_db.sh ===

// and go to
// === readme.md ===

// If everything is working you shoudl see something like this at the end of your terminal...

// + echo 'Postgres has been migrated, ready to go!'
// Postgres has been migrated, ready to go!

// // ⛳️ Step 2 - Adding A Migration

// // To store our subscribers details we need to create our first table.
// // To add a new table to our database we need to change its schema - this is commonly referred to as a database
// // migration.

// // run the following in your terminal
// export DATABASE_URL=postgres://app:secret@127.0.0.1:5432/newsletter
// sqlx migrate add create_subscriptions_table

// // A new top-level directory should have now appeared in your project - migrations. This is where all migrations for our project will be stored by sqlx’s CLI

// // Add this query to the {timestamp}_create_subscriptions_table.sql file:

// -- Create Subscriptions Table
// CREATE TABLE subscriptions(
// id uuid NOT NULL,
// PRIMARY KEY (id),
// email TEXT NOT NULL UNIQUE,
// name TEXT NOT NULL,
// subscribed_at timestamptz NOT NULL
// );

// // then run:
// sqlx migrate run

// ⛳️ Step 3 - View Database in a GUI client

// Go to
// === scripts/init_db.sh ===

// // ⛳️ Step 4 - Writing Our First Query

// // // Add this to cargo.toml
// // [dependencies.sqlx]
// // version= "0.8"
// // default-features = false
// // features = [
// // "runtime-tokio-rustls",
// // "macros",
// // "postgres",
// // "uuid",
// // "chrono",
// // "migrate"
// // ]

// // Using table-like toml syntax to avoid a super-long line!

// // One by one:
// // - runtime-tokio-rustls: Use tokio async runtime + rustls for TLS
// // - macros: Enable sqlx::query! and sqlx::query_as! macros
// // - postgres: Enable Postgres-specific types and SQL features
// // - uuid: Map SQL UUID columns to Rust Uuid type
// // - chrono: Map SQL timestamptz columns to Rust DateTime<T> type
// // - migrate: Access migration functions for tests

// // Go to
// // === src/lib.rs ===

// use day_30_database::run;
// use std::net::TcpListener;

// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind port");
//     run(listener)?.await
// }

// // Run cargo test
// //
// // Make sure cargo test comes out green before moving forward.
// //   ✅ test health_check_works ... ok
// //   ✅ test subscribe_returns_a_200_for_valid_form_data ... ok
// //   ✅ test subscribe_returns_a_400_when_data_is_missing ... ok

// // ⛳️ Step 5 - Reading A Configuration File

// // Go to
// // === src/configuration.rs ===

// // The we modify our main function to read configuration as its first step:
// use day_30_database::configuration::get_configuration;
// use day_30_database::startup::run;
// use std::net::TcpListener;
// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     // Panic if we can't read configuration
//     let configuration = get_configuration().expect("Failed to read configuration.");
//     // We have removed the hard-coded `8000` - it's now coming from our settings!
//     let address = format!("127.0.0.1:{}", configuration.application_port);
//     let listener = TcpListener::bind(address)?;
//     run(listener)?.await
// }

// // Adda config file a ./configuration.yaml and the cargo run shoudl compile

// // ⛳️ Step 6 - Connecting To Postgres

// // Go to
// // === src/configuration.rs ===
// // === tests/health_check.rs ===

// use day_30_database::configuration::get_configuration;
// use day_30_database::startup::run;
// use std::net::TcpListener;
// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     // Panic if we can't read configuration
//     let configuration = get_configuration().expect("Failed to read configuration.");
//     // We have removed the hard-coded `8000` - it's now coming from our settings!
//     let address = format!("127.0.0.1:{}", configuration.application_port);
//     let listener = TcpListener::bind(address)?;
//     run(listener)?.await
// }

// ⛳️ Step 7 - Making Our Test Assertion
// Go to
// === tests/health_check.rs ===

// // ⛳️ Step 8 - The Data Extractor
// // Go to
// // === tests/subscriptions.rs ===
// // === src/startup.rs ===

// use day_30_database::configuration::get_configuration;
// use day_30_database::startup::run;
// // use sqlx::{Connection, PgConnection};
// use sqlx::PgPool;
// use std::net::TcpListener;

// // #[tokio::main]
// // async fn main() -> Result<(), std::io::Error> {
// //     let configuration = get_configuration().expect("Failed to read configuration.");
// //     let connection = PgConnection::connect(&configuration.database.connection_string())
// //         .await
// //         .expect("Failed to connect to Postgres.");
// //     let address = format!("127.0.0.1:{}", configuration.application_port);
// //     let listener = TcpListener::bind(address)?;
// //     run(listener, connection)?.await
// // }

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

// // cargo build  ✅

// ⛳️ Step 9 - Updating Our Tests
// Go to
// === tests/health_check.rs ===

// ⛳️ Step 10 - Idempotency
// Go to
// === tests/health_check.rs ===

// use actix_web::{HttpResponse, web};
// use chrono::Utc;
// use sqlx::PgPool;
// use uuid::Uuid;

// #[derive(serde::Deserialize)]
// pub struct FormData {
//     email: String,
//     name: String,
// }

// pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
//     match sqlx::query!(
//         r#"
// INSERT INTO subscriptions (id, email, name, subscribed_at)
// VALUES ($1, $2, $3, $4)
// "#,
//         Uuid::new_v4(),
//         form.email,
//         form.name,
//         Utc::now()
//     )
//     .execute(pool.get_ref())
//     .await
//     {
//         Ok(_) => HttpResponse::Ok().finish(),
//         Err(e) => {
//             println!("Failed to execute query: {}", e);
//             HttpResponse::InternalServerError().finish()
//         }
//     }
// }

// // ⛳️ Step 2 - Instrumenting POST /subscriptions
// //[dependencies]
// // log= "0.4"

// use actix_web::{HttpResponse, web};
// use chrono::Utc;
// use sqlx::PgPool;
// use uuid::Uuid;

// #[derive(serde::Deserialize)]
// pub struct FormData {
//     email: String,
//     name: String,
// }

// pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
//     // 🦀 - Let's generate a random unique identifier
//     let request_id = Uuid::new_v4();
//     // We are using the same interpolation syntax of `println`/`print` here!
//     log::info!(
//         "request_id {} - Adding '{}' '{}' as a new subscriber.",
//         request_id,
//         form.email,
//         form.name
//     );
//     log::info!(
//         "request_id {} - Saving new subscriber details in the database",
//         request_id
//     );
//     match sqlx::query!(
//         r#"
// INSERT INTO subscriptions (id, email, name, subscribed_at)
// VALUES ($1, $2, $3, $4)
// "#,
//         Uuid::new_v4(),
//         form.email,
//         form.name,
//         Utc::now()
//     )
//     .execute(pool.get_ref())
//     .await
//     {
//         Ok(_) => {
//             // 🦀
//             log::info!(
//                 "request_id {} - New subscriber details have been saved",
//                 request_id
//             );
//             HttpResponse::Ok().finish()
//         }
//         Err(e) => {
//             // 🦀 we are using {:?}, the std::fmt::Debug format, to capture the query error.
//             log::error!(
//                 "request_id {} - Failed to execute query: {:?}",
//                 request_id,
//                 e
//             );
//             HttpResponse::InternalServerError().finish()
//         }
//     }
// }

// // ⛳️ Step 3 - Migrating From log To tracing
// // // [dependencies]
// // tracing= { version= "0.1", features = ["log"] }

// use actix_web::{HttpResponse, web};
// use chrono::Utc;
// use sqlx::PgPool;
// use uuid::Uuid;

// #[derive(serde::Deserialize)]
// pub struct FormData {
//     email: String,
//     name: String,
// }

// pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
//     // 🦀 - Spans, like logs, have an associated level
//     let request_id = Uuid::new_v4();
//     // `info_span` creates a span at the info-level
//     let request_span = tracing::info_span!(
//         "Adding a new subscriber.",
//         %request_id,
//         subscriber_email= %form.email,
//         subscriber_name= %form.name
//     );
//     // Using `enter` in an async function is a recipe for disaster!
//     // Bear with me for now, but don't do this at home.
//     // See the following section on `Instrumenting Futures`
//     let _request_span_guard = request_span.enter();

//     match sqlx::query!(
//         r#"
// INSERT INTO subscriptions (id, email, name, subscribed_at)
// VALUES ($1, $2, $3, $4)
// "#,
//         Uuid::new_v4(),
//         form.email,
//         form.name,
//         Utc::now()
//     )
//     .execute(pool.get_ref())
//     .await
//     {
//         Ok(_) => {
//             // 🦀
//             log::info!(
//                 "request_id {} - New subscriber details have been saved",
//                 request_id
//             );
//             HttpResponse::Ok().finish()
//         }
//         Err(e) => {
//             // 🦀 we are using {:?}, the std::fmt::Debug format, to capture the query error.
//             log::error!(
//                 "request_id {} - Failed to execute query: {:?}",
//                 request_id,
//                 e
//             );
//             HttpResponse::InternalServerError().finish()
//         }
//     }
// }

// // now run:
// //
// // RUST_LOG=trace cargo run
// //
// // Instead of INFO logs you shoudl see...
// // [2026-02-14T05:24:03Z TRACE actix_server::worker] starting server worker 0
// // [2026-02-14T05:24:03Z TRACE actix_web::middleware::logger] Access log format: %a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
// // ...

// ⛳️ Step 4 - Instrumenting Futures

use actix_web::{HttpResponse, web};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    // 🦀
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email= %form.email,
        subscriber_name= %form.name
    );
    let _request_span_guard = request_span.enter();

    // We do not call `.enter` on query_span!
    // `.instrument` takes care of it at the right moments
    // in the query future lifetime
    // let query_span = tracing::info_span!("Saving new subscriber details in the database");

    match sqlx::query!(
        r#"
INSERT INTO subscriptions (id, email, name, subscribed_at)
VALUES ($1, $2, $3, $4)
"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            // 🦀
            // Yes, this error log falls outside of `query_span`
            // We'll rectify it later, pinky swear!
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

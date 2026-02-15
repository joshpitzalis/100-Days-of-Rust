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

// // ⛳️ Step 2 - Cleaning Up Instrumentation Code
// // We’d like to wrap the subscribe function in a span.

// use actix_web::{HttpResponse, web};
// use chrono::Utc;
// use sqlx::PgPool;
// use tracing::Instrument;
// use uuid::Uuid;

// #[derive(serde::Deserialize)]
// pub struct FormData {
//     email: String,
//     name: String,
// }

// #[tracing::instrument(
//     name = "Adding a new subscriber",
//     skip(form, pool),
//     fields(
//         request_id= %Uuid::new_v4(),
//         subscriber_email= %form.email,
//         subscriber_name= %form.name
// ))]
// pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
//     // 🦀
//     let query_span = tracing::info_span!("Saving new subscriber details in the database");
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
//     // 🦀
//     .instrument(query_span)
//     .await
//     {
//         Ok(_) => HttpResponse::Ok().finish(),
//         Err(e) => {
//             tracing::error!("Failed to execute query: {:?}", e);
//             HttpResponse::InternalServerError().finish()
//         }
//     }
// }

// // ⛳️ Step 3 - Better separation of concerns
// // insert_subscriber takes care of the database logic and it has no awareness of the surrounding web framework - i.e. we are not passing web::Form or web::Data wrappers as input types;
// // • subscribe orchestrates the work to be done by calling the required routines and translates their outcome into the proper response according to the rules and conventions of the HTTP protocol.

// use actix_web::{HttpResponse, web};
// use chrono::Utc;
// use sqlx::PgPool;
// // use tracing::Instrument;
// use uuid::Uuid;

// #[derive(serde::Deserialize)]
// pub struct FormData {
//     email: String,
//     name: String,
// }

// #[tracing::instrument(
//     name = "Adding a new subscriber",
//     skip(form, pool),
//     fields(
//         request_id= %Uuid::new_v4(),
//         subscriber_email= %form.email,
//         subscriber_name= %form.name
// ))]
// pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
//     match insert_subscriber(&pool, &form).await {
//         Ok(_) => HttpResponse::Ok().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }

// #[tracing::instrument(
//     name = "Saving new subscriber details in the database",
//     skip(form, pool)
// )]
// pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
//     sqlx::query!(
//         r#"
// INSERT INTO subscriptions (id, email, name, subscribed_at)
// VALUES ($1, $2, $3, $4)
// "#,
//         Uuid::new_v4(),
//         form.email,
//         form.name,
//         Utc::now()
//     )
//     .execute(pool)
//     .await
//     .map_err(|e| {
//         tracing::error!("Failed to execute query: {:?}", e);
//         e
//         // Using the `?` operator to return early
//         // // if the function failed, returning a sqlx::Error
//         // We will talk about error handling in depth later!
//     })?;
//     Ok(())
// }

// // ⛳️ Step 5 - Ensure all logs for a particular request have a Request Id

// use actix_web::{HttpResponse, web};
// use chrono::Utc;
// use sqlx::PgPool;
// use uuid::Uuid;

// #[derive(serde::Deserialize)]
// pub struct FormData {
//     email: String,
//     name: String,
// }

// #[tracing::instrument(
//     name = "Adding a new subscriber",
//     skip(form, pool),
//     fields(
//         // 🦀
//         // request_id= %Uuid::new_v4(),
//         subscriber_email= %form.email,
//         subscriber_name= %form.name
// ))]
// pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
//     match insert_subscriber(&pool, &form).await {
//         Ok(_) => HttpResponse::Ok().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }

// #[tracing::instrument(
//     name = "Saving new subscriber details in the database",
//     skip(form, pool)
// )]
// pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
//     sqlx::query!(
//         r#"
// INSERT INTO subscriptions (id, email, name, subscribed_at)
// VALUES ($1, $2, $3, $4)
// "#,
//         Uuid::new_v4(),
//         form.email,
//         form.name,
//         Utc::now()
//     )
//     .execute(pool)
//     .await
//     .map_err(|e| {
//         tracing::error!("Failed to execute query: {:?}", e);
//         e
//         // Using the `?` operator to return early
//         // // if the function failed, returning a sqlx::Error
//         // We will talk about error handling in depth later!
//     })?;
//     Ok(())
// }

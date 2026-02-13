// use actix_web::{HttpResponse, web};

// #[derive(serde::Deserialize)]
// pub struct FormData {
//     email: String,
//     name: String,
// }

// pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
//     HttpResponse::Ok().finish()
// }

// ⛳️ Step 8 - The Data Extractor (dependency injection)

// [dependencies]
// uuid= { version= "1", features = ["v4"] }
// chrono = { version= "0.4.22", default-features = false, features = ["clock"] }

use actix_web::{HttpResponse, web};
use chrono::Utc;
// use sqlx::PgConnection;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
    // connection: web::Data<PgConnection>,
    pool: web::Data<PgPool>, // Renamed!
) -> HttpResponse {
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
    // // We use `get_ref` to get an immutable reference to the `PgConnection`
    // // wrapped by `web::Data`.
    // .execute(connection.get_ref())
    // .await;
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

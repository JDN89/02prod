
use actix_web::{HttpResponse, web};
use chrono::Utc;
use sqlx::{ PgPool};
use uuid::Uuid;

// In Rust, by default, items defined within a module are private and cannot be accessed from outside the module.
// If you want to make an item publicly accessible,
// you need to use the pub keyword in front of the item definition:

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>,
                       pool: web::Data<PgPool>,
) -> HttpResponse {
    // `Result` has two variants: `Ok` and `Err`.
// The first for successes, the second for failures.
// We use a `match` statement to choose what to do based
// on the outcome.
// We will talk more about `Result` going forward!
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
// We use `get_ref` to get an immutable reference to the `PgConnection`
// wrapped by `web::Data`.
        .execute(pool.get_ref())
        .await {
        Ok(_) => HttpResponse::Ok().finish(),

        Err(e) => {
            println!("Failed to execute the query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

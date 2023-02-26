use actix_web::{HttpResponse, web};
use chrono::Utc;
use sqlx::{PgPool};
use tracing::info;
use tracing::Instrument;
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
    let request_id = Uuid::new_v4();
// Spans, like logs, have an associated level
// `info_span` creates a span at the info-level
    let request_span = tracing::info_span!(
"Adding a new subscriber.",
%request_id,
subscriber_email = %form.email,
subscriber_name = %form.name
);
// Using `enter` in an async function is a recipe for disaster!
// Bear with me for now, but don't do this at home.
// See the following section on `Instrumenting Futures`
    let _request_span_guard = request_span.enter();

// We do not call `.enter` on query_span!
// `.instrument` takes care of it at the right moments
// in the query future lifetime
    let query_span = tracing::info_span!(
"Saving new subscriber details in the database"
);

// [...]
// `_request_span_guard` is dropped at the end of `subscribe`
// That's when we "exit" the span
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
        // First we attach the instrumentation, then we `.await` it
        .instrument(query_span)
        .await {
        Ok(_) => {
            info!(
"request_id {} - New subscriber details have been saved",
request_id
);
            HttpResponse::Ok().finish()
        }

        Err(e) => {
            // Yes, this error log falls outside of `query_span`
            // We'll rectify it later, pinky swear!
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

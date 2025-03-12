use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::{query, PgPool};
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
name="Adding a new subscriber",
skip(form, pool)
field(request_id = %Uuid::new_v4(),
    subcriber_name = %form.name,
    subcriber_email = %form.email,
))]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    match insert_subscriber(&form, &pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
name="Saving new subscriber to the database"
skip(form, pool)
)]
pub async fn insert_subscriber(form: &FormData, pool: &PgPool) -> sqlx::Result<(), sqlx::Error> {
    query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Should execute the query {:?}", e);
        e
    })?;
    Ok(())
}

use axum::{Json, http::{StatusCode}, extract::State};
use std::sync::Arc;
use sqlx::PgPool;
use crate::users::model::{User,NewUser};

pub async fn create_user (
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<NewUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    let result = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (name, email)
        VALUES ($1, $2)
        RETURNING id, name, email
        "#,
        payload.name,
        payload.email
    ).fetch_one(&*pool)
    .await
    .map_err(|err| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(Json(result))
} 
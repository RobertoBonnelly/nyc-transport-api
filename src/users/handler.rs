use axum::{Json, http::{StatusCode}, extract::State};
use std::sync::Arc;
use sqlx::{PgPool};
use crate::users::model::{User,NewUser};

/// Inserts a new user to the database
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

/// Updates an existing user in the datase by its id
pub async fn update_user (
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<User>,
) -> Result<Json<User>, (StatusCode, String)> {
    let result = sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET name=$2, email=$3
        WHERE id=$1
        RETURNING
            id,
            name,
            email
        "#,
        payload.id,
        payload.name,
        payload.email
    ).fetch_one(&*pool)
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(Json(result))
}

/// Deletes user by its id
pub async fn delete_user (
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<User>,
) -> Result<Json<User>, (StatusCode, String)> {
    let result = sqlx::query_as!(
        User,
        r#"
        DELETE FROM users
        WHERE id=$1
        RETURNING id, name, email 
        "#,
        payload.id
    ).fetch_one(&*pool)
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(Json(result))
}
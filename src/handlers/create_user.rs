use crate::db::db_insert::create_user;
use crate::db::models::{NewUser, User};
use axum::{Json, extract::State, http::StatusCode};
use sqlx::PgPool;

pub async fn create_user_handler(
    State(pool): State<PgPool>,
    Json(new_user): Json<NewUser>,
) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    let user = create_user(&pool, &new_user.name, &new_user.email)
        .await
        .map_err(|error| {
            eprintln!("failed to create user: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to create user".to_owned(),
            )
        })?;
    Ok((StatusCode::CREATED, Json(user)))
}

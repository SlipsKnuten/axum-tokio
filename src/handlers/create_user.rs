use crate::db::db_insert::create_user;
use crate::db::models::NewUser;
use axum::{Json, extract::State, http::StatusCode};
use sqlx::PgPool;

pub async fn create_user_handler(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<NewUser>>, (StatusCode, String)> {
    let users = create_user::create_user(&pool).await.map_err(|error| {
        eprintln!("failed to gets users: {error}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed got get users".to_owned(),
        )
    })?;
    Ok(Json(users))
}

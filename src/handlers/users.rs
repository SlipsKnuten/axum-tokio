use crate::db::models::User;
use crate::db::users::get_all_users;
use axum::{Json, extract::State, http::StatusCode};
use sqlx::PgPool;

pub async fn get_all(State(pool): State<PgPool>) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let users = get_all_users(&pool).await.map_err(|error| {
        eprintln!("failed to gets users: {error}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed got get users".to_owned(),
        )
    })?;
    Ok(Json(users))
}

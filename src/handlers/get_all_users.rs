use crate::db::models::User;
use crate::handlers::handlerconstants::GET_ALL_USERS_SQL_COMMAND;
use axum::{Json, extract::State, http::StatusCode};
use sqlx::PgPool;

pub async fn get_all_users(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let users = sqlx::query_as::<_, User>(GET_ALL_USERS_SQL_COMMAND)
        .fetch_all(&pool)
        .await
        .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;
    Ok(Json(users))
}

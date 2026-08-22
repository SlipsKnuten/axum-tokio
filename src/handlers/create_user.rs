use crate::db::db_insert::create_user;
use crate::db::models::NewUser;
use axum::{Json, extract::State, http::StatusCode};
use sqlx::PgPool;

pub async fn create_user_handler(
    State(pool): State<PgPool>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<Vec<NewUser>>, (StatusCode, String)> {
    let user = create_user(&pool, &new_user.email, &new_user.name)
        .await
        .map_err(|error| {
            eprintln!("failed to gets users: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed got get users".to_owned(),
            )
        })?;
    Ok((StatsCode::CREATED, Json(user)))
}

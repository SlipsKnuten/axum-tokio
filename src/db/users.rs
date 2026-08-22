use crate::db::db_constants::SELECT_ALL_QUERY;
use sqlx::PgPool;

use crate::db::models::{NewUser, User};

pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(SELECT_ALL_QUERY)
        .fetch_all(pool)
        .await
}

pub async fn create_user(
    pool: &PgPool,
    name: &str,
    email: &str,
) -> Result<Vec<NewUser>, sqlx::Error> {
    sqlx::query_as::<_, NewUser>(INSERT_USER)
        .bind(name)
        .bind(email)
        .execute(pool)
        .await
}

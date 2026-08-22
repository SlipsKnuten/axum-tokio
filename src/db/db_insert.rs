use crate::db::db_constants::INSERT_USER;
use sqlx::PgPool;

use crate::db::models::User;

pub async fn create_user(pool: &PgPool, name: &str, email: &str) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(INSERT_USER)
        .bind(name)
        .bind(email)
        .fetch_one(pool)
        .await
}

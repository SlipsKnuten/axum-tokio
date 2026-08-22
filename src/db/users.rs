use crate::db::db_constants::SELECT_ALL_QUERY;
use sqlx::PgPool;

use crate::db::models::User;

pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(SELECT_ALL_QUERY)
        .fetch_all(pool)
        .await
}

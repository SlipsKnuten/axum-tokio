use crate::db::db_constants::INSERT_USER;
use sqlx::PgPool;

use crate::db::models::NewUser;

pub async fn create_user(
    pool: &PgPool,
    name: &str,
    email: &str,
) -> Result<Vec<NewUser>, sqlx::Error> {
    sqlx::query_as::<_, NewUser>(INSERT_USER)
        .bind(&name)
        .bind(&email)
        .execute(pool)
        .await
    Ok(())
}

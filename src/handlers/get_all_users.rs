use crate::handlers::handlerconstants::GET_ALL_USERS_SQL_COMMAND;

pub async fn get_all_users(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(GET_ALL_USERS_SQL_COMMAND).execute(pool).await?;
    Ok(())
}

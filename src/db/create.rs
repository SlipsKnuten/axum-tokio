use sqlx::PgPool;

pub async fn create_users_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    println!("Users table ready");

    Ok(())
}

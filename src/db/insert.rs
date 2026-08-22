pub async fn db_insert(pool: &sqlx::PgPool, name: &str, email: &str) -> Result<(), sqlx::Error>{
    sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
        .bind(name)
        .bind(email)
        .execute(pool)
        .await?;
    Ok(())
}

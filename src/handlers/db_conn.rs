use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

pub async fn db_conn() -> Result<PgPool, sqlx::Error> {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DB URL not set");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&database_url)
        .await?;

    println!("Connected to db");

    Ok(pool)
}

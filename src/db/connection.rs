use dotenv::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

pub async fn connection() -> Result<PgPool, sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DB URL not set");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&database_url)
        .await?;

    println!("Connected to db");

    Ok(pool)
}

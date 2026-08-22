mod db;
mod handlers;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = db::connection().await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    db::create::create_users_table(&pool).await?;
    db::insert::db_insert(&pool, "Gustaf", "gustaf@example.com").await?;

    let app = routes::create_router().with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}

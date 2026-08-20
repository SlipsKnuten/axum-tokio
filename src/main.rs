mod handlers;
mod logging;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = handlers::db_conn().await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    handlers::db_insert::db_insert(&pool, "Gustaf", "gustaf@example.com").await?;

    let app = routes::create_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}

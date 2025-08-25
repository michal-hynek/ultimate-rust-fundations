use std::net::SocketAddr;

use axum::{routing::get, Extension, Router};

mod api;
mod collector;
mod web;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;

    let db_url = std::env::var("DB_URL")?;
    let pool = sqlx::SqlitePool::connect(&db_url).await?;

    let handle = tokio::spawn(collector::data_collector(pool.clone()));

    // Start the web server
    let app = Router::new()
        .route("/", get(get(web::index)))
        .route("/collector.html", get(web::collector))
        .route("/api/all", get(api::show_all))
        .route("/api/collectors", get(api::show_collectors))
        .route("/api/collectors/{uuid}", get(api::collector_data))
        .layer(Extension(pool));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service())
        .await?;

    // Wait for the data collector to finish
    handle.await??; // Two question marks - we're unwrapping the task result, and the result from running the collector.
    Ok(())
}


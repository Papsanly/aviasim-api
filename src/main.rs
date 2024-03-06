use std::env;
use axum::{Router, serve};
use axum::routing::get;
use tokio::net::TcpListener;
use color_eyre::Result;
use dotenvy::dotenv;
use sqlx::PgPool;

async fn hello_world() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;

    let url = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&url).await?;
    sqlx::migrate!().run(&pool).await?;

    let router = Router::new().route("/", get(hello_world));

    let listener = TcpListener::bind("0.0.0.0:3472").await?;
    serve(listener, router).await?;

    Ok(())
}

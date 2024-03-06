use axum::{routing::get, serve, Router};
use color_eyre::Result;
use sqlx::PgPool;
use std::env;
use tokio::net::TcpListener;

async fn hello_world() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, World!");

    let url = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&url).await?;
    sqlx::migrate!().run(&pool).await?;

    let router = Router::new().route("/", get(hello_world));

    let listener = TcpListener::bind("0.0.0.0:3472").await?;
    serve(listener, router).await?;

    Ok(())
}

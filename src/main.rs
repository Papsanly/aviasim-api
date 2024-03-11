mod error;
mod orm;

use crate::{error::ApiError, orm::Order};
use axum::{
    debug_handler,
    extract::State,
    http::StatusCode,
    response::Result,
    routing::{get, post},
    serve, Json, Router,
};
use sqlx::{query, PgPool};
use std::{env, sync::Arc};
use tokio::net::TcpListener;

async fn hello_world() -> &'static str {
    "Hello, World!"
}

#[debug_handler]
async fn order(
    State(pool): State<Arc<PgPool>>,
    Json(Order { order_info, time }): Json<Order>,
) -> Result<StatusCode, ApiError> {
    let mut transaction = pool.begin().await?;

    let id = query!(
        r#"
        insert into order_info (
            client_phone, client_email, client_name, client_provider, duration, simulator,
            payment_status
        )
        values ($1, $2, $3, $4, $5, $6, 'pending')
        returning id;
        "#,
        order_info.client.phone,
        order_info.client.email,
        order_info.client.name,
        order_info.client.provider.to_string(),
        order_info.duration,
        order_info.simulator.to_string()
    )
    .fetch_one(transaction.as_mut())
    .await?
    .id;

    query!(
        r#"
        insert into "order" (order_info, time) values ($1, $2)
        "#,
        id,
        time
    )
    .execute(transaction.as_mut())
    .await?;

    transaction.commit().await?;
    Ok(StatusCode::CREATED)
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let url = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&url).await?;
    sqlx::migrate!().run(&pool).await?;

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/order", post(order))
        .with_state(Arc::new(pool));

    let listener = TcpListener::bind("0.0.0.0:3472").await?;
    serve(listener, router).await?;

    Ok(())
}

use std::sync::{Arc, Mutex};

use axum::{routing::{get, post}, Router};
use tower_http::services::ServeDir;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::{routes::{home::home, hx::rewrite}, service::{ollama::OllamaRewriteService, rewrite_service::RewriteService}};


mod service;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");


    let port = std::env::var("PORT")?;

    let hx_router = Router::new()
        .route("/rewrite", post(rewrite));

    let router = Router::new()
        .route("/", get(home))
        .nest("/hx", hx_router)
        .nest_service("/static", ServeDir::new("./static"));

    info!("starting server on 0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    axum::serve(listener, router).await?;

    Ok(())
}

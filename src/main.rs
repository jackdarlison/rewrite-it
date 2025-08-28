use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use tokio::sync::Mutex;
use tower_http::services::ServeDir;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

use crate::{
    routes::{
        home::home,
        hx::{iterate, rewrite},
    },
    service::{ollama::OllamaRewriteService, rewrite_service::RewriteService},
};

mod routes;
mod service;

#[derive(Debug, Clone)]
struct AppState<RS: RewriteService> {
    rewrite_service: Arc<Mutex<RS>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let port = std::env::var("PORT")?;

    let ollama_url = std::env::var("OLLAMA_URL").unwrap_or("http://localhost".to_string());
    let ollama_service = OllamaRewriteService::new(ollama_url).await?;
    let state = AppState {
        rewrite_service: Arc::new(Mutex::new(ollama_service)),
    };

    let hx_router = Router::new()
        .route("/rewrite", post(rewrite))
        .route("/iterate", post(iterate));

    let router = Router::new()
        .route("/", get(home))
        .nest("/hx", hx_router)
        .with_state(state)
        .nest_service("/static", ServeDir::new("./static"));

    info!("starting server on 0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    axum::serve(listener, router).await?;

    Ok(())
}

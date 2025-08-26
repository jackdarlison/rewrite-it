use askama::Template;
use axum::{response::{Html, IntoResponse}, routing::get, Router};
use tower_http::services::ServeDir;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;


mod service;

#[derive(Template)]
#[template(path = "home.html")]
struct Home {

}

async fn home() -> impl IntoResponse {
    let template = Home {};

    Html(template.render().expect("err"))
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");


    let port = std::env::var("PORT")?;
    let ollama_url = std::env::var("OLLAMA_URL").unwrap_or("http://localhost".to_string());


    let router = Router::new()
        .route("/", get(home))
        .nest_service("/static", ServeDir::new("./static"));

    info!("starting server on 0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    axum::serve(listener, router).await?;

    Ok(())
}

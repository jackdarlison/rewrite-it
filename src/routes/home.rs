use askama::Template;
use axum::response::{Html, IntoResponse};
use ollama_rs::Ollama;

#[derive(Template)]
#[template(path = "home.html")]
struct Home {
    models: Vec<String>,
}

pub async fn home() -> impl IntoResponse {

    let ollama_url = std::env::var("OLLAMA_URL").unwrap_or("http://localhost".to_string());
    let ollama = Ollama::new(ollama_url, 11434);
    let models = match ollama.list_local_models().await {
        Ok(lms) => lms.iter().map(|lm| lm.name.clone()).collect(),
        Err(_) => return Html("Cannot find local models".to_string()),
    };

    let template = Home { models };

    Html(template.render().expect("err"))
}
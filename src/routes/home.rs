use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};

use crate::{AppState, service::rewrite_service::RewriteService};

#[derive(Template)]
#[template(path = "home.html")]
struct Home {
    models: Vec<String>,
}

pub async fn home<RS: RewriteService>(State(state): State<AppState<RS>>) -> impl IntoResponse {
    let service = state.rewrite_service.lock().await;
    let models = service.list_available_models().await;

    if models.is_empty() {
        return Html("<h1>Please ensure models are available!</h1>".to_string());
    }

    let template = Home { models };

    Html(template.render().expect("Home template should render!"))
}

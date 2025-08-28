use axum::{Form, Json, extract::State, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::{AppState, service::rewrite_service::RewriteService};

#[derive(Deserialize, Debug)]
pub struct RewriteForm {
    input_code: String,
    input_select: String,
    output_select: String,
    model_select: String,
}

#[derive(Deserialize, Debug)]
pub struct IterateForm {
    input_code: String,
    model_select: String,
    iterate_text: String,
}

#[derive(Serialize, Debug)]
pub struct Output {
    code: String,
    explanation: String,
}

pub async fn rewrite<RS: RewriteService>(
    State(state): State<AppState<RS>>,
    Form(data): Form<RewriteForm>,
) -> impl IntoResponse {
    dbg!(&data);

    let mut ollama_service = state.rewrite_service.lock().await;

    let out = ollama_service
        .start_rewrite(
            &data.input_code,
            &data.input_select,
            &data.output_select,
            &data.model_select,
        )
        .await;

    match out {
        Ok(r) => Json(Output {
            code: r.rewritten_code,
            explanation: r.explanation,
        }),
        Err(e) => Json(Output {
            code: "".to_string(),
            explanation: format!("Error generating response: {}", e),
        }),
    }
}

pub async fn iterate<RS: RewriteService>(
    State(state): State<AppState<RS>>,
    Form(data): Form<IterateForm>,
) -> impl IntoResponse {
    dbg!(&data);

    let mut ollama_service = state.rewrite_service.lock().await;

    let out = ollama_service
        .iterate(&data.input_code, &data.iterate_text, &data.model_select)
        .await;

    match out {
        Ok(r) => Json(Output {
            code: r.rewritten_code,
            explanation: r.explanation,
        }),
        Err(e) => Json(Output {
            code: "".to_string(),
            explanation: format!("Error iterating response: {}", e),
        }),
    }
}

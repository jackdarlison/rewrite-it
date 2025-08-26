use axum::{response::IntoResponse, Form, Json};
use serde::{Deserialize, Serialize};

use crate::{service::{ollama::OllamaRewriteService, rewrite_service::RewriteService}};


#[derive(Deserialize, Debug)]
pub struct RewriteForm {
    input_code: String,
    input_select: String,
    output_select: String,
    model_select: String,
}

#[derive(Serialize, Debug)]
pub struct Output {
    code: String,
    explanation: String,
}

pub async fn rewrite(Form(data): Form<RewriteForm>) -> impl IntoResponse {
    dbg!(&data);

    let ollama_url = std::env::var("OLLAMA_URL").unwrap_or("http://localhost".to_string());
    let ollama_service = OllamaRewriteService::new(ollama_url).await;

    let ollama_service = match ollama_service {
        Ok(s) => s,
        Err(e) => {
            return Json( Output { code: "".to_string(), explanation: format!("Could not create Ollama service: {}", e) });
        },
    };

    let out = ollama_service.start_rewrite(
        &data.input_code,
        &data.input_select,
        &data.output_select,
        &data.model_select,
    ).await;

    match out {
        Ok(r) => Json( Output { code: r.code, explanation: r.explanation }),
        Err(e) => Json( Output { code: "".to_string(), explanation: format!("Error generating response: {}", e) }),
    }
}
use axum::{response::IntoResponse, Form, Json};
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Debug)]
pub struct RewriteForm {
    input_code: String,
    input_select: String,
    output_select: String,
}

#[derive(Serialize, Debug)]
pub struct Output {
    code: String,
    explanation: String,
}

pub async fn rewrite(Form(data): Form<RewriteForm>) -> impl IntoResponse {
    dbg!(&data);

    let output = format!(
        "Input: {}\nLeft: {}\nRight: {}",
        data.input_code, data.input_select, data.output_select
    );

    Json(Output { code: output, explanation: "".to_string() })
}
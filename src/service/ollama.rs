use ollama_rs::{generation::{completion::request::GenerationRequest, parameters::{FormatType, JsonStructure}}, Ollama};

use crate::service::rewrite_service::{RewriteError, RewriteOutput, RewriteService};

#[derive(Debug, Clone)]
pub struct OllamaRewriteService {
    url: String,
    history: Vec<String>,
    
}

impl OllamaRewriteService {
    pub async fn new(url: String) -> Result<Self, RewriteError> {
        let this = OllamaRewriteService {
            url: url.clone(), history: vec![] };

        let ollama = Ollama::new(url, 11434);
        ollama.list_local_models().await.map_err(|e| RewriteError::InitialisationError(format!("{:?}", e)))?;

        Ok(this)
    }

}


impl RewriteService for OllamaRewriteService {
    async fn start_rewrite(&self, input: &str, from: &str, to: &str, using: &str) -> Result<RewriteOutput, RewriteError> {
        let prompt = format!(
            r#"You are a professional {to} developer.
            
            Please rewrite the following snippet, that is already in the {from} langauge, into the {to} language:
            
            ```
            {input}
            ```
            "#
        );

        let ollama = Ollama::new(self.url.clone(), 11434);
        
        
        let format = FormatType::StructuredJson(Box::new(JsonStructure::new::<RewriteOutput>()));

        let res = ollama
            .generate(
                GenerationRequest::new(using.to_string(), prompt)
                    .format(format)
                    // .options(ModelOptions::default().temperature(0.0)),
            )
            .await.map_err(|_| RewriteError::RewriteError("Error generating output".to_string()))?;

        let json_resp: RewriteOutput = serde_json::from_str(&res.response).map_err(|_e| RewriteError::StructuredDataError(res.response.clone()))?;

        dbg!(&json_resp);

        Ok(json_resp)
    }

    // async fn iterate(&mut self, aid: &str) -> Result<super::rewrite_service::RewriteOutput, super::rewrite_service::RewriteError> {
    //     todo!()
    // }
}
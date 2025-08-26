use ollama_rs::Ollama;

use crate::service::rewrite_service::{RewriteError, RewriteService};



pub struct OllamaRewriteService {
    url: String,
    history: Vec<String>,
    model: Option<String>,
    from: Option<String>,
    to: Option<String>,
}

impl OllamaRewriteService {
    pub async fn new(url: String) -> Result<Self, RewriteError> {
        let this = OllamaRewriteService {
            url: url.clone(), model: None, history: vec![], from: None, to: None };

        let ollama = Ollama::new(url, 11434);
        ollama.list_local_models().await.map_err(|e| RewriteError::InitialisationError(format!("{:?}", e)))?;

        Ok(this)
    }
}



impl RewriteService for OllamaRewriteService {
    async fn start_rewrite(input: &str, from: &str, to: &str, using: &str) -> Result<super::rewrite_service::RewriteOutput, super::rewrite_service::RewriteError> {
        todo!()
    }

    async fn iterate(aid: &str) -> Result<super::rewrite_service::RewriteOutput, super::rewrite_service::RewriteError> {
        todo!()
    }
}
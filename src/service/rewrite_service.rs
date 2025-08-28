use schemars::JsonSchema;
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RewriteError {
    #[error("Cannot initialise model: {0}")]
    InitialisationError(String),

    #[error("Cannot generate rewrite response: {0}")]
    RewriteError(String),

    #[error("Returned data is not structured: {0}")]
    StructuredDataError(String),

    #[error("User has tried to iterate with without starting a rewrite session!")]
    IterateWithNoRewriteError,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct RewriteOutput {
    pub rewritten_code: String,
    pub explanation: String,
}

pub trait RewriteService: Clone + Send + Sync + 'static {
    /// List all available local models
    fn list_available_models(&self) -> impl Future<Output = Vec<String>> + Send;

    /// Start a new rewrite
    fn start_rewrite(
        &mut self,
        input: &str,
        from: &str,
        to: &str,
        using: &str,
    ) -> impl Future<Output = Result<RewriteOutput, RewriteError>> + Send;

    /// Interate on a rewrite with history
    fn iterate(
        &mut self,
        input: &str,
        iterate: &str,
        using: &str,
    ) -> impl Future<Output = Result<RewriteOutput, RewriteError>> + Send;
}

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
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct RewriteOutput {
    pub code: String,
    pub explanation: String,
}


pub trait RewriteService: Clone + Send + Sync + 'static {

    /// Start a new rewrite
    fn start_rewrite(&self, input: &str, from: &str, to: &str, using: &str) -> impl Future<Output = Result<RewriteOutput, RewriteError>> + Send;

    // Interate on a rewrite with history
    // async fn iterate(&mut self, aid: &str) -> Result<RewriteOutput, RewriteError>;
}

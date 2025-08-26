use thiserror::Error;



#[derive(Error, Debug)]
pub enum RewriteError {
    #[error("Cannot initialise model: {0}")]
    InitialisationError(String),

}

pub struct RewriteOutput {
    code: String,
    exmplanation: String,
}


pub trait RewriteService {

    /// Start a new rewrite
    async fn start_rewrite(input: &str, from: &str, to: &str, using: &str) -> Result<RewriteOutput, RewriteError>;

    /// Interate on a rewrite with history
    async fn iterate(aid: &str) -> Result<RewriteOutput, RewriteError>;
}

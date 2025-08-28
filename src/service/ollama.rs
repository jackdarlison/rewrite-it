use ollama_rs::{
    Ollama,
    generation::{
        chat::{ChatMessage, request::ChatMessageRequest},
        parameters::{FormatType, JsonStructure},
    },
};

use crate::service::rewrite_service::{RewriteError, RewriteOutput, RewriteService};

#[derive(Debug, Clone)]
pub struct OllamaRewriteService {
    url: String,
    ollama: Ollama,
    history: Vec<ChatMessage>,
    iterate_info: Option<IterateInfo>,
}

#[derive(Debug, Clone)]
struct IterateInfo {
    from: String,
    to: String,
}

impl OllamaRewriteService {
    pub async fn new(url: String) -> Result<Self, RewriteError> {
        let ollama = Ollama::new(url.clone(), 11434);
        ollama
            .list_local_models()
            .await
            .map_err(|e| RewriteError::InitialisationError(format!("{:?}", e)))?;

        let this = OllamaRewriteService {
            url: url.clone(),
            ollama,
            history: vec![],
            iterate_info: None,
        };

        Ok(this)
    }
}

impl RewriteService for OllamaRewriteService {
    async fn list_available_models(&self) -> Vec<String> {
        self.ollama
            .list_local_models()
            .await
            .map_or(vec![], |lms| lms.iter().map(|lm| lm.name.clone()).collect())
    }

    async fn start_rewrite(
        &mut self,
        input: &str,
        from: &str,
        to: &str,
        using: &str,
    ) -> Result<RewriteOutput, RewriteError> {
        let prompt = format!(
            r#"You are a professional {to} developer.
            
            Please rewrite the following code snippet written in the {from} langauge, into the {to} language:
            
            ```
            {input}
            ```
            "#
        );

        let format = FormatType::StructuredJson(Box::new(JsonStructure::new::<RewriteOutput>()));

        self.history.clear();

        let res = self
            .ollama
            .send_chat_messages_with_history(
                &mut self.history,
                ChatMessageRequest::new(using.to_string(), vec![ChatMessage::user(prompt)])
                    .format(format), // .options(ModelOptions::default().temperature(0.0)),
            )
            .await
            .map_err(|_| RewriteError::RewriteError("Error generating output".to_string()))?;

        let json_resp: RewriteOutput = serde_json::from_str(&res.message.content)
            .map_err(|_e| RewriteError::StructuredDataError(res.message.content.clone()))?;
        dbg!(&json_resp);

        self.iterate_info = Some(IterateInfo {
            from: from.to_string(),
            to: to.to_string(),
        });

        Ok(json_resp)
    }

    async fn iterate(
        &mut self,
        input: &str,
        iterate: &str,
        using: &str,
    ) -> Result<RewriteOutput, RewriteError> {
        let iterate_info = self
            .iterate_info
            .as_ref()
            .ok_or(RewriteError::IterateWithNoRewriteError)?;
        let (from, to) = (&iterate_info.from, &iterate_info.to);

        let prompt = format!(
            r#"Continue rewriting the user's input prompt written in {from}, it is given here as a reminder:

            ```
            {input}
            ```

            The user has given the following feedback:

            "{iterate}"

            please rewrite the code snippet into {to} again, using the feedback to guide you.
            "#
        );

        let format = FormatType::StructuredJson(Box::new(JsonStructure::new::<RewriteOutput>()));

        let res = self
            .ollama
            .send_chat_messages_with_history(
                &mut self.history,
                ChatMessageRequest::new(using.to_string(), vec![ChatMessage::user(prompt)])
                    .format(format), // .options(ModelOptions::default().temperature(0.0)),
            )
            .await
            .map_err(|_| RewriteError::RewriteError("Error generating output".to_string()))?;

        let json_resp: RewriteOutput = serde_json::from_str(&res.message.content)
            .map_err(|_e| RewriteError::StructuredDataError(res.message.content.clone()))?;

        dbg!(&json_resp);

        Ok(json_resp)
    }
}

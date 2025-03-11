use crate::{Result, config::Config, error::Error};
use async_recursion::async_recursion;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::time::Duration;

pub const DEFAULT_CODE_MODEL: &str = "qwen2.5-coder:3b";
pub const DEFAULT_CODE_NUM_CTX: u32 = 4096;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    pub content: String,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct OllamaRequest {
    pub messages: Vec<Message>,
    pub model: String,
    pub options: Options,
    pub stream: bool,
}

#[derive(Debug, Deserialize)]
pub struct OllamaResponse {
    message: Message,
}

#[derive(Debug, Serialize)]
pub struct Options {
    pub num_ctx: u32,
}

#[async_recursion]
pub async fn request(config: Arc<Config>, messages: Vec<Message>) -> Result<String> {
    let options = Options {
        num_ctx: DEFAULT_CODE_NUM_CTX,
    };

    let model = config
        .model
        .clone()
        .unwrap_or(DEFAULT_CODE_MODEL.to_string());

    let ollama_request = OllamaRequest {
        messages: messages.clone(),
        model,
        options,
        stream: false,
    };

    let url = format!("{}/api/chat", config.ollama_host);

    let response = reqwest::ClientBuilder::new()
        .connect_timeout(Duration::from_secs(240))
        .timeout(Duration::from_secs(240))
        .build()?
        .post(url)
        .json(&ollama_request)
        .send()
        .await;

    match response {
        Err(e) => {
            eprintln!("Error: {e}");

            let response = request(config, messages).await;

            return response;
        }
        Ok(response) => {
            if response.status() == StatusCode::CREATED || response.status() == StatusCode::OK {
                let response_text = response.text().await?;

                let ollama_response: OllamaResponse = serde_json::from_str(&response_text)?;

                return Ok(ollama_response.message.content);
            }
        }
    }

    Err(Box::new(Error::OllamaRequestProblem))
}

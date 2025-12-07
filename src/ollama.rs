use crate::{Result, app, app::Mode, config::Config, error::Error};
use async_recursion::async_recursion;
use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::time::Duration;

pub const DEFAULT_CODE_MODEL: &str = "qwen3-coder:30b";
pub const DEFAULT_CODE_NUM_CTX: u32 = 16384;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    pub content: String,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct OllamaRequest {
    pub keep_alive: u16,
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
    pub temperature: f32,
}

#[async_recursion]
pub async fn request(
    config: Arc<Config>,
    messages: Vec<Message>,
    num_ctx: Option<u32>,
    attempt: u8,
) -> Result<String> {
    if attempt > config.max_attempts {
        return Ok(String::new());
    }

    let options = Options {
        num_ctx: num_ctx.unwrap_or(DEFAULT_CODE_NUM_CTX),
        temperature: 0.0,
    };

    let model = config
        .model
        .clone()
        .unwrap_or(DEFAULT_CODE_MODEL.to_string());

    let ollama_request = OllamaRequest {
        keep_alive: config.keep_alive,
        messages: messages.clone(),
        model: model.clone(),
        options,
        stream: false,
    };

    let url = format!("{}/api/chat", config.ollama_host);

    let response = reqwest::ClientBuilder::new()
        .connect_timeout(Duration::from_secs(config.timeout))
        .timeout(Duration::from_secs(config.timeout))
        .build()?
        .post(url)
        .json(&ollama_request)
        .send()
        .await;

    match response {
        Err(e) => {
            eprintln!("Error: {e}");

            let attempt = attempt + 1;

            let response = request(config, messages, num_ctx, attempt).await;

            return response;
        }
        Ok(response) => {
            if response.status() == StatusCode::CREATED || response.status() == StatusCode::OK {
                let response_text = response.text().await?;

                let ollama_response: OllamaResponse = serde_json::from_str(&response_text)?;

                let signature = app::signature(&model);

                let response = format!("{}\n\n{signature}", ollama_response.message.content);

                return Ok(response);
            }
        }
    }

    Err(Box::new(Error::OllamaRequestProblem))
}

pub async fn run_request(
    config: Arc<Config>,
    messages: Vec<Message>,
    start_date: DateTime<Utc>,
) -> Result<()> {
    let mut length = 0;

    for message in &messages {
        length += message.content.len();
    }

    let num_ctx = (u32::try_from(length)? / 4) + 4096;

    println!(
        "Context window = {num_ctx}\tkeep_alive = {}\ttimeout = {}\n\n",
        config.keep_alive, config.timeout
    );

    if let Some(skip_larger) = config.skip_larger
        && num_ctx > skip_larger
    {
        println!("Context too large. Skipping...");

        return Ok(());
    }

    let result = request(config.clone(), messages.clone(), Some(num_ctx), 1).await?;

    println!("{result}");

    let end_date = Utc::now();

    let delta = end_date - start_date;

    let task = match config.mode {
        Mode::Ask => "Answer generated",
        Mode::Checker => "Checked",
        Mode::CommitReview => "Commit review generated",
        Mode::CommitSummary => "Commit summary generated",
        Mode::CriteriaVerify => "Criteria verified",
        Mode::Explain => "Explained",
        Mode::Performance => "Checked",
        Mode::TaskGenerate => "Task generated",
        Mode::TaskReview => "Task review generated",
    };

    println!(
        "\n\n{task} in {} seconds.\n",
        delta.num_seconds()
    );

    Ok(())
}

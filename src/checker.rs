use crate::{
    Result,
    config::Config,
    ollama::{self, Message},
};
use std::sync::Arc;

pub const SYSTEM_PROMPT: &str = "Your role is code auditing.
You will receive a fragment of code of a larger project.";

pub const SYSTEM_PROMPT_FINDING_PROBLEMS: &str = "You need to find problems with this code.
Order problems from more to less serious.";

pub const SYSTEM_PROMPT_ANSWER_TEMPLATE: &str =
    "Use the following template for describing problems:
==========
Problem summary

Problem detailed description

Optional sample code that triggers an error";

pub async fn run(config: Arc<Config>, code: &str) -> Result<String> {
    let mut messages = vec![];

    let message = Message {
        content: SYSTEM_PROMPT.to_string(),
        role: "system".to_string(),
    };
    messages.push(message);

    let message = Message {
        content: SYSTEM_PROMPT_FINDING_PROBLEMS.to_string(),
        role: "system".to_string(),
    };
    messages.push(message);

    let message = Message {
        content: SYSTEM_PROMPT_ANSWER_TEMPLATE.to_string(),
        role: "system".to_string(),
    };
    messages.push(message);

    let prompt = format!("Here is the code: {code}");
    let message = Message {
        content: prompt,
        role: "user".to_string(),
    };
    messages.push(message);

    let response = ollama::request(config.clone(), messages.clone()).await?;

    Ok(response)
}

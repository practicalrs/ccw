use crate::{
    Result,
    config::Config,
    ollama::{self, Message},
};
use std::sync::Arc;

pub const SYSTEM_PROMPT: &str = "Your role is code auditing.
You will receive a fragment of the code of a larger project.
Don't point out issues that you are not 100% sure are bugs.
Only comment when you find something suspicious. Otherwise, say that the code looks ok.";

pub const SYSTEM_PROMPT_FINDING_PROBLEMS: &str = "You need to find problems with this code.
Order problems from more to less serious.
Only point out problems that are:
- cryptographic errors
- documentation errors
- logic errors
- overflow errors
- security bugs
- unsafe code bugs
Or similar problems that may lead to security-related problems.";

pub const SYSTEM_PROMPT_ANSWER_TEMPLATE: &str =
    "Use the following template for describing problems:
==========
Problem summary

Problem detailed description

Optional sample code that triggers an error
==========";

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
        content: prompt.clone(),
        role: "user".to_string(),
    };
    messages.push(message);

    let mut length = 0;
    length += SYSTEM_PROMPT.len();
    length += SYSTEM_PROMPT_FINDING_PROBLEMS.len();
    length += SYSTEM_PROMPT_ANSWER_TEMPLATE.len();
    length += prompt.len();

    let num_ctx = (length as u32 / 4) + 4096;

    println!("num_ctx = {num_ctx}");

    let response = ollama::request(config.clone(), messages.clone(), Some(num_ctx)).await?;

    Ok(response)
}

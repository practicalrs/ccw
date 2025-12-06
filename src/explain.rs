use crate::{
    Result,
    config::Config,
    ollama::{self, Message},
};
use chrono::Utc;
use std::sync::Arc;

pub const SYSTEM_PROMPT: &str = "You are a code-analysis assistant. The user will provide a string containing either a large amount of code or a small code fragment. Your job is to clearly explain what the provided code does and answer any optional user question.

Follow these rules:
- Always begin with a high-level overview of what the code is intended to do (based solely on what is provided).
- Then provide a deeper explanation of important structures such as functions, classes, data flows, algorithms, or key logic.
- If the user provides only a small or incomplete snippet, focus on explaining the visible logic and discuss what can be reasonably inferred.
- If the user asks a specific question, answer it directly and thoroughly after explaining the code.
- Avoid adding functionality not present in the text; keep your inferences grounded and clearly noted.
- Use clear, concise, developer-friendly language.
- When relevant, point out noteworthy patterns, potential issues, performance concerns, or unusual design choices.

Your goal is to help the user fully understand the given code, regardless of its size or completeness.";

pub async fn run(config: Arc<Config>, code: &str) -> Result<()> {
    let start_date = Utc::now();

    let mut messages = vec![];

    let message = Message {
        content: SYSTEM_PROMPT.to_string(),
        role: "system".to_string(),
    };
    messages.push(message);

    if let Some(question) = &config.question {
        let prompt = format!("Here is the question about the code: {question}");
        let message = Message {
            content: prompt.clone(),
            role: "user".to_string(),
        };
        messages.push(message);
    }

    let prompt = format!("Here is the code: {code}");
    let message = Message {
        content: prompt.clone(),
        role: "user".to_string(),
    };
    messages.push(message);

    let mut length = 0;
    length += SYSTEM_PROMPT.len();
    length += prompt.len();

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

    let result = ollama::request(config.clone(), messages.clone(), Some(num_ctx), 1).await?;

    println!("{result}");

    let end_date = Utc::now();

    let delta = end_date - start_date;
    println!("Explained in {} seconds.\n", delta.num_seconds());

    Ok(())
}

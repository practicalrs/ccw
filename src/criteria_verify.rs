use crate::{
    Result,
    config::Config,
    file::read,
    ollama::{self, Message},
};
use chrono::Utc;
use std::sync::Arc;

pub const SYSTEM_PROMPT: &str = "You are CCW-CRITERIA-VERIFY. Your role is to determine whether code changes meet the acceptance criteria.

You will receive:
- A list of acceptance criteria for a task.
- A diff from a larger project.

Follow these rules:

1. Evaluate each acceptance criterion strictly against what is visible in the diff.
   - Do not rely on assumptions or inferred context.
   - Only observable code changes count as evidence.

2. For every criterion, output exactly one of the following:
   - “Met” — the diff clearly and fully satisfies the criterion.
   - “Not Met” — the diff does not satisfy the criterion.
   - “Partially Met” — the diff satisfies some, but not all, required elements.
   - “Not Verifiable” — the diff does not contain enough information to evaluate the criterion.

3. After each result, include a brief explanation grounded in the diff:
   - Reference the nature of changes (added function, modified validation, new API call, etc.).
   - Do NOT quote large parts of the diff.
   - Do NOT speculate about behaviors not visible in the provided code.

4. Do not reinterpret, expand, or modify the acceptance criteria.
   - Evaluate them exactly as written.
   - Do not introduce new requirements or assumptions.

5. At the end, include a short summary stating:
   - whether all criteria are met,
   - or whether some or all criteria are not met.

6. Do not include meta-commentary, process notes, or disclaimers.
   Output only the evaluation.

Your goal is to deliver a strict, objective, diff-based assessment of whether the code changes fulfill the acceptance criteria.";

pub async fn run(config: Arc<Config>, code: &str) -> Result<()> {
    let start_date = Utc::now();

    let mut messages = vec![];

    let message = Message {
        content: SYSTEM_PROMPT.to_string(),
        role: "system".to_string(),
    };
    messages.push(message);

    if let Some(file) = &config.file {
        let criteria_content = read(&config, file)?;

        let content = format!("Here are the acceptance criteria: {criteria_content}");

        let message = Message {
            content,
            role: "system".to_string(),
        };
        messages.push(message);
    } else {
        println!("You need to provide input --file with acceptance criteria.");

        return Ok(());
    }

    let prompt = format!("Here is the code: {code}");
    let message = Message {
        content: prompt.clone(),
        role: "user".to_string(),
    };
    messages.push(message);

    ollama::run_request(config, messages, start_date).await?;

    Ok(())
}

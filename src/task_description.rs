use crate::{
    Result,
    config::Config,
    ollama::{self, Message},
};
use chrono::Utc;
use std::sync::Arc;

pub const SYSTEM_PROMPT: &str = "Your role is to summarize code changes as a task description.

You will receive a diff from a larger project. Your output must follow these rules:

1. Begin the output with a short, clear task title (one line, plain text, no markdown headings).
2. After the title, provide a structured task description.
3. Markdown is allowed in the task description (lists, paragraphs, inline code).
4. Do not quote large pieces of the diff.
5. Describe only changes that actually appear in the diff; do not speculate.
6. Focus on what was changed, added, removed, refactored, or fixed.
7. If the diff is short, create a brief paragraph summary.
8. If the diff is longer, create a logically grouped list of changes.
9. Keep the description factual and concise. No commentary, no motivations unless clearly implied by the diff.
10. After the task description, generate an \"Acceptance Criteria\" section in markdown list form.
11. Acceptance criteria must describe observable, testable outcomes strictly derived from the diff.
12. Acceptance criteria must not include speculative behavior beyond what the code change actually implements.
13. Do not add disclaimers or meta-comments. Only output the final summary.

Your goal is to create a task-style summary with clear acceptance criteria suitable for issue trackers.

If you'd like, I can also produce an example output format so your tool can be easily tested with known diff samples.";

pub async fn run(config: Arc<Config>, code: &str) -> Result<()> {
    let start_date = Utc::now();

    let mut messages = vec![];

    let message = Message {
        content: SYSTEM_PROMPT.to_string(),
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
    length += prompt.len();

    let num_ctx = (u32::try_from(length)? / 4) + 4096;

    println!("Context window = {num_ctx}\tkeep_alive = {}\ttimeout = {}\n\n", config.keep_alive, config.timeout);

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
    println!("\n\nTask description generated in {} seconds.\n", delta.num_seconds());

    Ok(())
}

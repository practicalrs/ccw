use crate::{
    Result,
    config::Config,
    ollama::{self, Message},
};
use chrono::Utc;
use std::sync::Arc;

pub const SYSTEM_PROMPT: &str = "You are CCW-TASK-DESCRIPTION. Your role is to summarize code changes as a task description.

You will receive a diff from a larger project. Your output must follow these rules:

1. Begin with a short, clear task title.
   - One line only.
   - Plain text (no markdown headings, no special formatting).

2. After the title, produce a structured task description.
   - Markdown is allowed (paragraphs, lists, inline code).
   - Do not quote large sections of the diff.
   - Describe only what is actually changed, added, removed, refactored, or fixed.
   - Do not speculate about behavior not visible in the diff.
   - If the diff is small, provide a brief paragraph summary.
   - If the diff is large, provide a grouped and logically organized list of changes.

3. Keep all descriptions factual and concise.
   - No commentary.
   - No justifications or speculation.
   - No interpretation of developer intentions.

4. After the task description, create a section titled “Acceptance Criteria” using Markdown.
   - Each criterion must be an observable, testable outcome strictly derived from the diff.
   - No speculative behavior.
   - No requirements that are not implemented in the visible changes.
   - Criteria should be phrased so that a reviewer can verify them using the code in the diff.

5. Do not include disclaimers, meta-comments, or process explanations.
   Output only the final task description with acceptance criteria.

Your goal is to generate a clear, reviewer-ready task summary suitable for issue trackers.";

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
    println!(
        "\n\nTask description generated in {} seconds.\n",
        delta.num_seconds()
    );

    Ok(())
}

use crate::{
    Result,
    config::Config,
    ollama::{self, Message},
};
use chrono::Utc;
use std::sync::Arc;

pub const SYSTEM_PROMPT: &str = "Your role is to summarize code changes as a task comment.

You will receive a diff from a larger project. Your output must follow these rules:

1. Produce a clear and concise task comment describing the changes.
2. Markdown is allowed (paragraphs, lists, inline code).
3. Only describe changes that actually appear in the diff; do not speculate.
4. Do not quote large portions of the diff.
5. Focus on what was changed, added, removed, fixed, or refactored.
6. Include a separate section titled \"How to Test\" describing:
   - What parts of the system should be tested
   - How the reviewer or QA can verify the changes
   - Steps or conditions needed to confirm correct behavior
   - Edge cases or failure modes worth checking
   The testing section should be based only on the diff, without guessing implementation details.
7. Keep the description factual and free of meta-commentary or disclaimers.
8. Only output the final comment.

Your goal is to produce a clear, reviewer-friendly comment suitable for issue trackers.";

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

    println!("Context window = {num_ctx}\n\n");

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
    println!("\n\nSummarized in {} seconds.\n", delta.num_seconds());

    Ok(())
}

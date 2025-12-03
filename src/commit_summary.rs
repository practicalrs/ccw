use crate::{
    Result,
    config::Config,
    ollama::{self, Message},
};
use chrono::Utc;
use std::sync::Arc;

pub const SYSTEM_PROMPT: &str = "Your role is to summarize code changes for commit messages.

You will receive a diff from a larger project.
Your output must follow these rules:

1. Produce a commit title using the Conventional Commits format: type(optional scope): concise description. The title must be a single line and must accurately reflect the primary change in the diff.
2. Choose the commit type based strictly on the diff content. Use feat for new features or added functionality. Use fix for bug fixes or corrections. Use refactor for internal code changes that do not change behavior. Use perf for performance improvements. Use docs for documentation changes. Use test for test-only changes. Use chore for maintenance tasks. Use ci for changes to continuous integration configurations or scripts. Use build for build system changes. Use style for formatting changes that do not affect behavior. Use revert when the diff reverts previous changes.
3. If the change introduces a breaking change, append an exclamation mark after the type or after type(scope) in the title.
4. After the title, produce a plain text summary of the changes.
5. Do not use markdown of any kind. Avoid symbols such as #, *, -, _, `, or any formatting syntax.
6. Do not include code blocks or quote the diff directly.
7. The summary must contain only neutral, factual descriptions of the changes.
8. If the diff is short, produce one to three concise sentences.
9. If the diff is longer, produce a list of changes using plain text lines separated by newlines. Do not use bullets, dashes, or numbering; simply separate items with newlines.
10. Do not invent changes not present in the diff.
11. Never include characters or formatting that could be interpreted as a Git comment.

Only output the commit title and the summary. Do not add commentary or disclaimers.";

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
    println!("\n\nCommit summary generated in {} seconds.\n", delta.num_seconds());

    Ok(())
}

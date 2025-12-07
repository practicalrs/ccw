use crate::{
    Result,
    config::Config,
    ollama::{self, Message},
};
use chrono::Utc;
use std::sync::Arc;

pub const SYSTEM_PROMPT: &str = "You are CCW-COMMIT-SUMMARY, a precise commit summarizer. You receive a code diff and produce a Conventional Commits–style title and a factual, concise summary of the actual changes. You must stay strictly grounded in the diff and must not invent or infer behavior that is not shown.

Your responsibilities:
1. Generate a commit title in the format: type(optional scope): description
2. The title must:
   - Accurately reflect the primary change visible in the diff
   - Use the correct Conventional Commit type based solely on the diff:
       feat: New functionality or visible feature
       fix: Bug fix or correctness correction
       refactor: Internal changes without behavior change
       perf: Performance-related changes
       docs: Documentation-only changes
       test: Test-only changes
       chore: Maintenance, internal tooling, config (not CI)
       ci: CI pipeline configuration changes
       build: Build system, compiler flags, dependencies
       style: Formatting-only (no behavior change)
       revert: Reverts a previous commit
   - Include an exclamation mark after type or type(scope) if the diff introduces a breaking change
   - Be a single line, maximum 42 characters
3. After the title, produce a plain text summary of the changes:
   - No markdown, no bullets, no special symbols (#, *, -, _, `).
   - No quoting the diff or including code blocks.
   - Only factual descriptions of what the diff changes.
   - No speculation, no inferred behavior.
4. Summary length rules:
   - For small diffs, use one to three concise sentences.
   - For large diffs, produce multiple separate plain-text lines:
       Each line must describe one high-level change.
       Separate lines with a blank newline.
       Do not use bullets, dashes, or numbering.
5. Line length limits:
   - Commit title ≤ 42 characters.
   - Summary lines ≤ 72 characters.
6. Do not include anything aside from the title and summary.
7. Do not output commentary, meta-information, disclaimers, or explanations.

If no meaningful change is present in the diff, still produce a valid commit title and summary describing that no code changes occurred.";

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
        "\n\nCommit summary generated in {} seconds.\n",
        delta.num_seconds()
    );

    Ok(())
}

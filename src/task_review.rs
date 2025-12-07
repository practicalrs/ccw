use crate::{
    Result,
    config::Config,
    ollama::{self, Message},
};
use chrono::Utc;
use std::sync::Arc;

pub const SYSTEM_PROMPT: &str = "You are CCW-TASK-REVIEW. Your job is to summarize code changes as a clear, concise task comment.

You will receive a diff from a larger project. Follow these rules:

1. Summarize only what is explicitly shown in the diff.
   - Describe additions, removals, modifications, and refactors.
   - Do NOT speculate about unseen behavior or unrelated parts of the system.
   - Do NOT invent context or intentions not supported by the diff.

2. Keep the summary factual and reviewer-friendly.
   - Markdown is allowed (paragraphs, bullet lists, inline code).
   - Do not quote large sections of the diff.
   - Focus on describing *what* changed, not *why*, unless the reason is directly visible in the diff.

3. After the summary, include a section titled:
   ## How to Test
   This section must:
   - Describe what areas or features should be tested based solely on the diff.
   - Provide steps or criteria the reviewer/QA can use to verify correctness.
   - Mention edge cases or failure modes only if identifiable from the diff.

4. Do not add meta-commentary, disclaimers, or process notes.
5. Output only the final task comment.

Your output structure:

<summary of changes>

## How to Test
<testing instructions grounded strictly in the diff>";

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

    ollama::run_request(config, messages, start_date).await?;

    Ok(())
}

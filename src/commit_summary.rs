use crate::{
    Result,
    config::Config,
    ollama::{self, Message},
};
use chrono::Utc;
use std::sync::Arc;

pub const SYSTEM_PROMPT: &str = "Your role is code summarizing.
You will receive a diff of the code of a larger project.
Summarize the code changes that it makes.
If the diff is short, try to summarize it in one or three sentences.
If the diff is longer, try to summarize changes in points.";

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

    if let Some(skip_larger) = config.skip_larger {
        if num_ctx > skip_larger {
            println!("Context too large. Skipping...");

            return Ok(());
        }
    }

    let result = ollama::request(config.clone(), messages.clone(), Some(num_ctx), 1).await?;

    println!("{result}");

    let end_date = Utc::now();

    let delta = end_date - start_date;
    println!("\n\nSummarized in {} seconds.\n", delta.num_seconds());

    Ok(())
}

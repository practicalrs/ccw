use crate::{
    Result,
    config::Config,
    ollama::{self, Message},
};
use chrono::Utc;
use std::sync::Arc;

pub const SYSTEM_PROMPT: &str = "You are CCW-EXPLAIN, a precise code-analysis assistant. You will receive a fragment of code from a larger project. Your task is to give a clear, structured explanation of what the provided code does and optionally answer a user-supplied question.

Your responsibilities:
1. Produce a high-level overview of the code's purpose based solely on what is visible.
2. Provide a deeper explanation of important elements such as:
   - functions, types, structs, enums, traits, classes
   - data flow, control flow, state changes
   - algorithms or key logic paths
3. When the snippet is small or incomplete:
   - Focus on what is explicitly present
   - Clearly distinguish between *observations from the code* and *inferences*
   - Mark inferences explicitly (e.g., “Based on naming, this may… but the snippet does not show it”)
4. If the user includes a specific question:
   - Answer it directly after the explanation
   - Base your answer only on observable or clearly justified inferences
5. Do NOT:
   - Invent functionality not present
   - Assume external context not shown
   - Speculate about code behavior without grounding
6. Use clear, concise, technically accurate language appropriate for developers.
7. When relevant (but only when visible from the code), briefly note:
   - design choices
   - possible pitfalls
   - potential performance concerns
   - unusual or notable idioms
   These must always be grounded in what is present in the snippet.

Output structure:
1. High-level overview
2. Detailed explanation
3. (Optional) Answer to the user’s specific question

Your goal is to help the user fully understand the given code while staying accurate, grounded, and free of speculation.";

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

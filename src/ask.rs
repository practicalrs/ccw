use crate::{
    Result,
    config::Config,
    ollama::{self, Message},
};
use chrono::Utc;
use std::sync::Arc;

pub const SYSTEM_PROMPT: &str = "You are CCW-ASK, a high-precision technical advisor. Your purpose is to answer software engineering questions that are not directly tied to a specific code snippet or diff. You provide accurate, concise, actionable guidance with minimal speculation.

Your responsibilities:
1. Explain concepts clearly using precise technical language.
2. Provide practical steps or patterns when applicable.
3. Generate example code ONLY when the user requests code or when an example is essential to understanding.
4. For incomplete or broad questions, outline the missing assumptions and present the most common valid approaches without hallucinating details.
5. If a topic involves unsafe or low-level behavior (OS dev, bootloaders, memory layouts, concurrency), be explicit about limitations, requirements, and cautions.
6. Avoid vague generalities — prioritize specifics, constraints, and tradeoffs.
7. Do not invent APIs, libraries, or language features that do not exist. If something may vary by version or platform, say so.
8. Keep responses structured:
   - Summary (1–2 sentences)
   - Key concepts or prerequisites
   - Step-by-step explanation or actionable steps
   - Example (only if requested or needed)
   - Additional considerations (warnings / common pitfalls)

Your goal: Provide expert-level, trustworthy, implementable answers to technical questions.";

pub async fn run(config: Arc<Config>) -> Result<()> {
    let start_date = Utc::now();

    let mut messages = vec![];

    let message = Message {
        content: SYSTEM_PROMPT.to_string(),
        role: "system".to_string(),
    };
    messages.push(message);

    if let Some(question) = &config.question {
        let prompt = format!("Here is the question: {question}");
        let message = Message {
            content: prompt.clone(),
            role: "user".to_string(),
        };
        messages.push(message);
    } else {
        println!("You need to provide --question with the question you want to ask.");

        return Ok(());
    }

    ollama::run_request(config, messages, start_date).await?;

    Ok(())
}

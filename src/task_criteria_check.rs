use crate::{
    Result,
    config::Config,
    file::read,
    ollama::{self, Message},
};
use chrono::Utc;
use std::sync::Arc;

pub const SYSTEM_PROMPT: &str = "Your role is to check if code changes meet the acceptance criteria.

You will receive:
- A list of acceptance criteria written for the task.
- A diff from a larger project.

Your output must follow these rules:

1. Evaluate each acceptance criterion strictly against what is present in the diff.
2. For every criterion, output one of the following results:
   - \"Met\" — the diff clearly satisfies the criterion.
   - \"Not Met\" — the diff does not satisfy the criterion.
   - \"Partially Met\" — the diff satisfies some parts but not all.
3. For each result, include a short explanation referencing observable elements in the diff.
4. Do not quote large pieces of the diff; refer to changes in general terms only.
5. Do not speculate about behavior not shown in the diff.
6. Do not guess developer intentions.
7. Do not introduce new requirements or reinterpret the existing criteria.
8. If a criterion cannot be evaluated from the diff, mark it as \"Not Verifiable\" and explain why.
9. At the end, output a short summary stating whether all criteria are met.
10. Do not add commentary, disclaimers, or meta-analysis. Only output the evaluation.

Your goal is to provide an objective and reliable assessment of whether the code changes satisfy the given acceptance criteria.";

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
        "\n\nTask criteria check generated in {} seconds.\n",
        delta.num_seconds()
    );

    Ok(())
}

use crate::{
    Result,
    config::Config,
    ollama::{self, Message},
};
use chrono::Utc;
use std::sync::Arc;

pub const SYSTEM_PROMPT: &str = "Your role is code auditing.
You will receive a fragment of the code of a larger project.
Only comment when you find something suspicious.
Suspicious means:
- incorrect security assumptions
- dangerous API usage
- misuse of cryptography
- unsafely handling external input
- race conditions or concurrency hazards
- unsafe code blocks that can violate memory safety
- panic / unwrap / expect in sensitive paths
- integer overflows or unchecked arithmetic
- flawed access control or authorization logic
- dangerous file system or network usage
- exposed secrets or keys
Otherwise, say that the code looks ok.";

pub const SYSTEM_PROMPT_FINDING_PROBLEMS: &str = "You need to find problems with this code.
Order problems from more to less serious.
Only point out problems such as:
- cryptographic vulnerabilities or misuse
- logic errors affecting security or correctness
- overflow and boundary errors
- unsafe Rust usage that may violate memory safety
- misuse of unwrap(), expect(), or panic in production paths
- insecure error handling
- concurrency or race condition bugs
- insecure file system or network access patterns
- serialization / deserialization vulnerabilities
- documentation inaccuracies affecting safe API usage
- any issue that may lead to security vulnerabilities
Or similar problems that may lead to security-related problems.";

pub const SYSTEM_PROMPT_ANSWER_TEMPLATE: &str =
    "Use the following template for describing problems:
==========
Problem summary
(A short title capturing the security issue.)

Problem detailed description
- Why this is a security problem
- How it can occur
- What conditions or inputs trigger it
- Potential impact/severity

Relevant code snippet or simplified example (optional)

Recommendation for how to fix the issue (required)
==========";

pub async fn run(config: Arc<Config>, code: &str) -> Result<()> {
    let start_date = Utc::now();

    let mut messages = vec![];

    let message = Message {
        content: SYSTEM_PROMPT.to_string(),
        role: "system".to_string(),
    };
    messages.push(message);

    let message = Message {
        content: SYSTEM_PROMPT_FINDING_PROBLEMS.to_string(),
        role: "system".to_string(),
    };
    messages.push(message);

    let message = Message {
        content: SYSTEM_PROMPT_ANSWER_TEMPLATE.to_string(),
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
    length += SYSTEM_PROMPT_FINDING_PROBLEMS.len();
    length += SYSTEM_PROMPT_ANSWER_TEMPLATE.len();
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
    println!("Checked in {} seconds.\n", delta.num_seconds());

    Ok(())
}

use crate::{
    Result,
    config::Config,
    ollama::{self, Message},
};
use chrono::Utc;
use std::sync::Arc;

pub const SYSTEM_PROMPT: &str = "Your role is code auditing.

You will receive a fragment of code from a larger project.
Your task is to find performance issues only.

A performance issue is something that measurably increases:
- CPU usage
- memory usage
- allocation count
- I/O overhead
- locking / contention
- algorithmic complexity

Do NOT comment on:
- formatting or style
- naming conventions
- readability unless it clearly affects performance
- unproven micro-optimizations
- speculative issues without evidence

Only comment when you find a real and meaningful performance problem.
Otherwise, say that the code looks ok.";

pub const SYSTEM_PROMPT_FINDING_PROBLEMS: &str =
    "You need to find problems with the performance of this code.

Order problems from more serious to less serious.

Examples of issues you should report:
- unnecessary heap allocations
- allocations inside loops
- repeated cloning (clone, to_owned, to_string) when borrowing is possible
- expensive operations inside loops (regex creation, hashing, sorting, formatting)
- failure to use reserve(), with_capacity(), or pre-allocation
- unnecessary temporary collections
- inefficient algorithms (e.g., O(n^2) loops working on large data)
- unbuffered I/O operations
- locking or contention issues
- blocking calls inside async code
- misuse of data structures (e.g., using Vec instead of HashSet for lookups)
- excessive conversions or trait object dispatch
- repeated deserialization or parsing
- any pattern that clearly increases CPU, memory, or I/O cost

Only report meaningful performance issues. If unsure, state that the information is insufficient.";

pub const SYSTEM_PROMPT_ANSWER_TEMPLATE: &str =
    "Use the following template for describing problems:
==========
Problem summary

Problem detailed description
- Why this affects performance
- What patterns or inputs make it worse
- Estimated impact (high / medium / low)

Recommended fix

Optional code example
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
    println!("Checked in {} seconds.\n", delta.num_seconds());

    Ok(())
}

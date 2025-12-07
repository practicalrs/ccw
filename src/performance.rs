use crate::{
    Result,
    config::Config,
    ollama::{self, Message},
};
use chrono::Utc;
use std::sync::Arc;

pub const SYSTEM_PROMPT: &str = "You are CCW-PERFORMANCE, a strict performance auditor. You will receive a fragment of source code from a larger project. Your task is to identify only performance-related issues visible in the provided code.

Your responsibilities:
1. Report only real, observable performance issues grounded in the code.
2. If no meaningful performance problems are visible, say the code looks ok.
3. Order problems from most serious to least serious.
4. Use the required structured template for each reported issue.

A performance issue is a pattern that measurably increases:
- CPU usage
- memory usage
- allocation count or allocation frequency
- I/O overhead
- locking, contention, or blocking
- algorithmic complexity

You must NOT comment on:
- formatting, style, or naming
- readability unless it directly impacts performance
- micro-optimizations without clear measurable benefit
- hypothetical problems not supported by the code
- speculative risks or imagined usage patterns

Only report issues when the code clearly demonstrates:
- unnecessary heap allocations
- allocations inside loops
- repeated cloning (clone, to_owned, to_string) where borrowing is possible
- expensive operations inside loops (regex creation, sorting, hashing, formatting)
- missed opportunities for reserve(), with_capacity(), or preallocation
- unnecessary intermediate collections or transformations
- inefficient algorithms (e.g., O(n^2) on large data)
- unbuffered I/O operations
- blocking calls in async contexts
- locking or contention issues
- inappropriate data structure choice (e.g., Vec for frequent membership checks)
- excessive conversions or trait-object dispatch
- repeated parsing, deserialization, or similar work
- any pattern that clearly increases CPU, memory, or I/O cost

If the information is insufficient to determine a performance impact, write:
“Information insufficient to assess performance.”

Use this exact template for each reported problem:

==========
Problem summary

Problem detailed description
- Why this affects performance
- What patterns or inputs make it worse
- Estimated impact (high / medium / low)

Recommended fix

Optional code example
==========

Output only your findings in the required format. No commentary outside the template.";

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
    println!("Checked in {} seconds.\n", delta.num_seconds());

    Ok(())
}

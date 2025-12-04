use crate::{
    Result,
    config::Config,
    ollama::{self, Message},
};
use chrono::Utc;
use std::sync::Arc;

pub const SYSTEM_PROMPT: &str = "Your role is to review a commit by analyzing the provided code diff.

Your goal is to identify issues related to:
1. Security and correctness
2. Performance
3. Missing documentation
4. Missing or insufficient tests
5. Opportunities to make the solution more universal or reusable

Follow these rules:

Security & Correctness:
- Report incorrect security assumptions
- Detect dangerous API usage
- Identify misuse of cryptography
- Flag unsafe handling of external input
- Identify race conditions or concurrency hazards
- Detect unsafe Rust code that may violate memory safety
- Flag panic/unwrap/expect in sensitive or production paths
- Catch integer overflows or unchecked arithmetic
- Identify flawed access control or authorization logic
- Flag insecure filesystem or network usage
- Identify exposed secrets or credentials
- Identify serialization/deserialization vulnerabilities
- Catch logic errors affecting correctness or safety

Performance:
- Report unnecessary heap allocations
- Allocations or expensive work inside loops
- Repeated cloning (clone, to_owned, to_string) when borrowing is possible
- Missing reserve() / with_capacity() for collections
- Unbuffered I/O operations
- Blocking calls in async code
- Misuse of data structures
- Excessive conversions or trait object dispatch
- Any code pattern that clearly increases CPU, memory, or I/O cost

Documentation:
- Point out missing or outdated documentation for public APIs
- Identify missing usage notes or safety invariants
- Identify undocumented assumptions or preconditions

Testing:
- Identify missing tests for new logic or edge cases
- Point out weak or insufficient coverage implied by the diff
- Suggest concrete behaviors that should be tested

Generality & Reusability:
- Suggest ways the code could be made more universal or robust
- Identify hard-coded values that should be parameters
- Suggest abstractions or reusable components when appropriate
- Identify strongly coupled structures that could be decoupled

Output Rules:
- Order findings from most serious to least serious.
- Only report meaningful issues. Do not speculate without evidence.
- Do not comment on style, formatting, or naming unless it affects correctness or performance.
- Only describe problems visible from the diff. Do not invent code not shown.
- If no meaningful issues are present, say the code looks ok.

Only output the final review findings.";

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

    println!("Context window = {num_ctx}\ttimeout = {}\n\n", config.timeout);

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
    println!("\n\nCommit review generated in {} seconds.\n", delta.num_seconds());

    Ok(())
}

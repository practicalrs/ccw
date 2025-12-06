#![forbid(unsafe_code)]

use std::error::Error;

mod app;
mod checker;
mod commit_review;
mod commit_summary;
mod config;
mod error;
mod explain;
mod file;
mod ollama;
mod performance;
mod task_comment;
mod task_criteria_check;
mod task_description;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let result = app::run();

    if let Err(e) = result.await {
        eprint!("Error: {e:?}");
    }

    Ok(())
}

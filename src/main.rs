#![forbid(unsafe_code)]

use std::error::Error;

mod app;
mod ask;
mod checker;
mod commit_review;
mod commit_summary;
mod config;
mod criteria_verify;
mod error;
mod explain;
mod file;
mod ollama;
mod performance;
mod task_generate;
mod task_review;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let result = app::run();

    if let Err(e) = result.await {
        eprint!("Error: {e:?}");
    }

    Ok(())
}

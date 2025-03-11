#![forbid(unsafe_code)]

use std::error::Error;

mod app;
mod config;
mod error;
mod file;
mod ollama;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let result = app::run();

    if let Err(e) = result.await {
        eprint!("Error: {e:?}");
    }

    Ok(())
}

use crate::{Result, config, file};
use clap::Parser;
use std::sync::Arc;

#[derive(Debug, Parser)]
#[command(about, author, long_about = None, version)]
pub struct Args {
    /// End line
    #[arg(long, short)]
    pub end_line: u32,

    /// File
    #[arg(long, short)]
    pub file: String,

    /// Ollama model
    #[arg(long, short)]
    pub model: Option<String>,

    /// Start line
    #[arg(long, short)]
    pub start_line: u32,
}

pub async fn run() -> Result<()> {
    let args = Args::parse();
    let config = Arc::new(config::load(args)?);

    let content = file::read(config.clone()).await?;

    println!("content = {content}");

    Ok(())
}

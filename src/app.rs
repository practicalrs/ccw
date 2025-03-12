use crate::{Result, checker, config, file};
use chrono::Utc;
use clap::Parser;
use std::sync::Arc;

#[derive(Debug, Parser)]
#[command(about, author, long_about = None, version)]
pub struct Args {
    /// End line
    #[arg(long, short)]
    pub end_line: Option<u32>,

    /// File
    #[arg(long, short)]
    pub file: String,

    /// Ollama model
    #[arg(long, short)]
    pub model: Option<String>,

    /// Start line
    #[arg(long, short)]
    pub start_line: Option<u32>,
}

pub async fn run() -> Result<()> {
    let start_date = Utc::now();

    let args = Args::parse();
    let config = Arc::new(config::load(args)?);

    let code = file::read(&config)?;
    let result = checker::run(config, &code).await?;

    println!("{result}");

    let end_date = Utc::now();

    let delta = end_date - start_date;
    println!("delta = {}", delta.num_seconds());

    Ok(())
}

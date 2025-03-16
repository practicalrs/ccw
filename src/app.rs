use crate::{Result, checker, config, file};
use clap::Parser;
use std::sync::Arc;

#[derive(Debug, Parser)]
#[command(about, author, long_about = None, version)]
pub struct Args {
    /// Dir
    #[arg(long, short)]
    pub dir: Option<String>,

    /// End line
    #[arg(long, short)]
    pub end_line: Option<u32>,

    /// File
    #[arg(long, short)]
    pub file: Option<String>,

    /// Ollama model
    #[arg(long, short)]
    pub model: Option<String>,

    /// Skip larger than tokens
    #[arg(long)]
    pub skip_larger: Option<u32>,

    /// Start line
    #[arg(long, short)]
    pub start_line: Option<u32>,
}

pub async fn run() -> Result<()> {
    let args = Args::parse();
    let config = Arc::new(config::load(args)?);

    let files = file::read_files(&config)?;

    for (file_name, code) in files {
        println!("file_name = {file_name}");

        checker::run(config.clone(), &code).await?;
    }

    Ok(())
}

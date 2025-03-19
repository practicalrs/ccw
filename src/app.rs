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

    /// Max attempts
    #[arg(long)]
    pub max_attempts: Option<u8>,

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
    let files_count = files.len();
    let mut i = 1;

    for (file_name, code) in files {
        println!("File {i} of {files_count} {file_name}");

        checker::run(config.clone(), &code).await?;

        i += 1;
    }

    Ok(())
}

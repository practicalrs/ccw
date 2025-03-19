use crate::{Result, checker, config, error::Error, file, performance};
use clap::Parser;
use std::{str::FromStr, sync::Arc};

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

    /// Mode
    #[arg(long)]
    pub mode: Option<String>,

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

#[derive(Clone, Debug)]
pub enum Mode {
    Checker,
    Performance,
}

impl FromStr for Mode {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lowercase = s.to_string().to_lowercase();
        let s = lowercase.as_str();
        match s {
            "checker" => Ok(Mode::Checker),
            "performance" => Ok(Mode::Performance),
            _ => Ok(Mode::Checker),
        }
    }
}

pub async fn run() -> Result<()> {
    let args = Args::parse();
    let config = Arc::new(config::load(args)?);

    let files = file::read_files(&config)?;
    let files_count = files.len();
    let mut i = 1;

    for (file_name, code) in files {
        println!("File {i} of {files_count} {file_name}");

        match config.mode {
            Mode::Checker => checker::run(config.clone(), &code).await?,
            Mode::Performance => performance::run(config.clone(), &code).await?,
        }

        i += 1;
    }

    Ok(())
}

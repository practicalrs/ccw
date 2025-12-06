use crate::{
    Result, checker, commit_review, commit_summary, config, explain, error::Error, file, performance, task_comment, task_criteria_check, task_description
};
use clap::Parser;
use std::{
    io::{Read, stdin},
    str::FromStr,
    sync::Arc,
};

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

    /// Keep alive in seconds
    #[arg(long, short)]
    pub keep_alive: Option<u16>,

    /// Max attempts
    #[arg(long)]
    pub max_attempts: Option<u8>,

    /// Mode
    #[arg(long)]
    pub mode: Option<String>,

    /// Ollama model
    #[arg(long, short)]
    pub model: Option<String>,

    /// Question
    #[arg(long, short)]
    pub question: Option<String>,

    /// Skip larger than tokens
    #[arg(long)]
    pub skip_larger: Option<u32>,

    /// Start line
    #[arg(long, short)]
    pub start_line: Option<u32>,

    /// Timeout
    #[arg(long, short)]
    pub timeout: Option<u64>,
}

#[derive(Clone, Debug)]
pub enum Mode {
    Checker,
    CommitReview,
    CommitSummary,
    Explain,
    Performance,
    TaskComment,
    TaskCriteriaCheck,
    TaskDescription,
}

impl FromStr for Mode {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lowercase = s.to_string().to_lowercase();
        let s = lowercase.as_str();
        match s {
            "checker" => Ok(Mode::Checker),
            "commit_review" => Ok(Mode::CommitReview),
            "commit_summary" => Ok(Mode::CommitSummary),
            "explain" => Ok(Mode::Explain),
            "performance" => Ok(Mode::Performance),
            "task_comment" => Ok(Mode::TaskComment),
            "task_criteria_check" => Ok(Mode::TaskCriteriaCheck),
            "task_description" => Ok(Mode::TaskDescription),
            _ => Ok(Mode::Checker),
        }
    }
}

pub async fn run() -> Result<()> {
    let args = Args::parse();
    let config = Arc::new(config::load(args)?);

    match config.mode {
        Mode::Checker | Mode::Explain | Mode::Performance => {
            let files = file::read_files(&config)?;
            let files_count = files.len();
            let mut i = 1;

            for (file_name, code) in files {
                println!("File {i} of {files_count} {file_name}");

                match config.mode {
                    Mode::Checker => checker::run(config.clone(), &code).await?,
                    Mode::Explain => explain::run(config.clone(), &code).await?,
                    Mode::Performance => performance::run(config.clone(), &code).await?,
                    _ => {}
                }

                i += 1;
            }
        }
        Mode::CommitReview | Mode::CommitSummary | Mode::TaskComment | Mode::TaskCriteriaCheck | Mode::TaskDescription => {
            let mut code = String::new();
            stdin().read_to_string(&mut code)?;

            match config.mode {
                Mode::CommitReview => commit_review::run(config.clone(), &code).await?,
                Mode::CommitSummary => commit_summary::run(config.clone(), &code).await?,
                Mode::TaskComment => task_comment::run(config.clone(), &code).await?,
                Mode::TaskCriteriaCheck => task_criteria_check::run(config.clone(), &code).await?,
                Mode::TaskDescription => task_description::run(config.clone(), &code).await?,
                _ => {}
            }
        }
    }

    Ok(())
}

pub fn signature(model: &str) -> String {
    let name = env!("CARGO_PKG_NAME").to_string();
    let version = env!("CARGO_PKG_VERSION").to_string();

    format!("Text generated with {name} (v{version})/{model}")
}

use crate::{
    Result, ask, checker, commit_review, commit_summary, config, criteria_verify, design_advice,
    error::Error, explain, file, performance, task_generate, task_review,
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
    Ask,
    Checker,
    CommitReview,
    CommitSummary,
    CriteriaVerify,
    DesignAdvice,
    Explain,
    Performance,
    TaskGenerate,
    TaskReview,
}

impl FromStr for Mode {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lowercase = s.to_string().to_lowercase();
        let s = lowercase.as_str();
        match s {
            "ask" => Ok(Mode::Ask),
            "checker" => Ok(Mode::Checker),
            "commit_review" => Ok(Mode::CommitReview),
            "commit_summary" => Ok(Mode::CommitSummary),
            "criteria_verify" => Ok(Mode::CriteriaVerify),
            "design_advice" => Ok(Mode::DesignAdvice),
            "explain" => Ok(Mode::Explain),
            "performance" => Ok(Mode::Performance),
            "task_generate" => Ok(Mode::TaskGenerate),
            "task_review" => Ok(Mode::TaskReview),
            _ => Ok(Mode::Checker),
        }
    }
}

pub async fn run() -> Result<()> {
    let args = Args::parse();
    let config = Arc::new(config::load(args)?);

    match config.mode {
        Mode::Checker | Mode::DesignAdvice | Mode::Explain | Mode::Performance => {
            let files = file::read_files(&config)?;
            let files_count = files.len();
            let mut i = 1;

            for (file_name, code) in files {
                println!("File {i} of {files_count} {file_name}");

                match config.mode {
                    Mode::Checker => checker::run(config.clone(), &code).await?,
                    Mode::DesignAdvice => design_advice::run(config.clone(), &code).await?,
                    Mode::Explain => explain::run(config.clone(), &code).await?,
                    Mode::Performance => performance::run(config.clone(), &code).await?,
                    _ => {}
                }

                i += 1;
            }
        }
        Mode::CommitReview
        | Mode::CommitSummary
        | Mode::CriteriaVerify
        | Mode::TaskGenerate
        | Mode::TaskReview => {
            let mut code = String::new();
            stdin().read_to_string(&mut code)?;

            match config.mode {
                Mode::CommitReview => commit_review::run(config.clone(), &code).await?,
                Mode::CommitSummary => commit_summary::run(config.clone(), &code).await?,
                Mode::CriteriaVerify => criteria_verify::run(config.clone(), &code).await?,
                Mode::TaskGenerate => task_generate::run(config.clone(), &code).await?,
                Mode::TaskReview => task_review::run(config.clone(), &code).await?,
                _ => {}
            }
        }
        Mode::Ask => match config.mode {
            Mode::Ask => ask::run(config.clone()).await?,
            _ => {}
        },
    }

    Ok(())
}

pub fn signature(model: &str) -> String {
    let name = env!("CARGO_PKG_NAME").to_string();
    let version = env!("CARGO_PKG_VERSION").to_string();

    format!("Text generated with {name} (v{version})/{model}")
}

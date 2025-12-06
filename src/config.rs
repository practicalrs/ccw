use crate::{
    Result,
    app::{Args, Mode},
    error::Error,
};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Config {
    pub dir: Option<String>,
    pub end_line: Option<u32>,
    pub file: Option<String>,
    pub keep_alive: u16,
    pub max_attempts: u8,
    pub mode: Mode,
    pub model: Option<String>,
    pub ollama_host: String,
    pub question: Option<String>,
    pub skip_larger: Option<u32>,
    pub start_line: Option<u32>,
    pub timeout: u64,
}

impl Config {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        dir: Option<String>,
        end_line: Option<u32>,
        file: Option<String>,
        keep_alive: u16,
        max_attempts: u8,
        mode: Mode,
        model: Option<String>,
        ollama_host: String,
        question: Option<String>,
        skip_larger: Option<u32>,
        start_line: Option<u32>,
        timeout: u64,
    ) -> Self {
        Self {
            dir,
            end_line,
            file,
            keep_alive,
            max_attempts,
            mode,
            model,
            ollama_host,
            question,
            skip_larger,
            start_line,
            timeout,
        }
    }
}

pub fn load(args: Args) -> Result<Config> {
    let dir = args.dir;
    let end_line = args.end_line;
    let file = args.file;
    let keep_alive = args.keep_alive.unwrap_or(0);
    let max_attempts = args.max_attempts.unwrap_or(3);
    let mode = if let Some(mode) = args.mode {
        Mode::from_str(&mode)?
    } else {
        Mode::Checker
    };
    let model = args.model;
    let Ok(ollama_host) = std::env::var("OLLAMA_HOST") else {
        return Err(Box::new(Error::OllamaHostAddresMissing));
    };
    let question = args.question;
    let skip_larger = args.skip_larger;
    let start_line = args.start_line;
    let timeout = args.timeout.unwrap_or(300);

    let config = Config::new(
        dir,
        end_line,
        file,
        keep_alive,
        max_attempts,
        mode,
        model,
        ollama_host,
        question,
        skip_larger,
        start_line,
        timeout,
    );

    Ok(config)
}

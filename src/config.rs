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
    pub max_attempts: u8,
    pub mode: Mode,
    pub model: Option<String>,
    pub ollama_host: String,
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
        max_attempts: u8,
        mode: Mode,
        model: Option<String>,
        ollama_host: String,
        skip_larger: Option<u32>,
        start_line: Option<u32>,
        timeout: u64,
    ) -> Self {
        Self {
            dir,
            end_line,
            file,
            max_attempts,
            mode,
            model,
            ollama_host,
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
    let skip_larger = args.skip_larger;
    let start_line = args.start_line;
    let timeout = args.timeout.unwrap_or(300);

    let config = Config::new(
        dir,
        end_line,
        file,
        max_attempts,
        mode,
        model,
        ollama_host,
        skip_larger,
        start_line,
        timeout,
    );

    Ok(config)
}

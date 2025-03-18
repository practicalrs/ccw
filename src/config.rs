use crate::{Result, app::Args, error::Error};

#[derive(Clone, Debug)]
pub struct Config {
    pub dir: Option<String>,
    pub end_line: Option<u32>,
    pub file: Option<String>,
    pub max_attempts: u8,
    pub model: Option<String>,
    pub ollama_host: String,
    pub skip_larger: Option<u32>,
    pub start_line: Option<u32>,
}

impl Config {
    pub fn new(
        dir: Option<String>,
        end_line: Option<u32>,
        file: Option<String>,
        max_attempts: u8,
        model: Option<String>,
        ollama_host: String,
        skip_larger: Option<u32>,
        start_line: Option<u32>,
    ) -> Self {
        Self {
            dir,
            end_line,
            file,
            max_attempts,
            model,
            ollama_host,
            skip_larger,
            start_line,
        }
    }
}

pub fn load(args: Args) -> Result<Config> {
    let dir = args.dir;
    let end_line = args.end_line;
    let file = args.file;
    let max_attempts = args.max_attempts.unwrap_or(3);
    let model = args.model;
    let Ok(ollama_host) = std::env::var("OLLAMA_HOST") else {
        return Err(Box::new(Error::OllamaHostAddresMissing));
    };
    let skip_larger = args.skip_larger;
    let start_line = args.start_line;

    let config = Config::new(
        dir,
        end_line,
        file,
        max_attempts,
        model,
        ollama_host,
        skip_larger,
        start_line,
    );

    Ok(config)
}

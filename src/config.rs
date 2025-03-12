use crate::{Result, app::Args, error::Error};

#[derive(Clone, Debug)]
pub struct Config {
    pub end_line: Option<u32>,
    pub file: String,
    pub model: Option<String>,
    pub ollama_host: String,
    pub start_line: Option<u32>,
}

impl Config {
    pub fn new(
        end_line: Option<u32>,
        file: String,
        model: Option<String>,
        ollama_host: String,
        start_line: Option<u32>,
    ) -> Self {
        Self {
            end_line,
            file,
            model,
            ollama_host,
            start_line,
        }
    }
}

pub fn load(args: Args) -> Result<Config> {
    let end_line = args.end_line;
    let file = args.file;
    let model = args.model;
    let Ok(ollama_host) = std::env::var("OLLAMA_HOST") else {
        return Err(Box::new(Error::OllamaHostAddresMissing));
    };
    let start_line = args.start_line;

    let config = Config::new(end_line, file, model, ollama_host, start_line);

    Ok(config)
}

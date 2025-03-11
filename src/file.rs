use crate::{Result, config::Config};
use std::{fs::read_to_string, sync::Arc};

pub async fn read(config: Arc<Config>) -> Result<String> {
    let mut result = String::new();
    let file_content = read_to_string(config.file.clone())?;

    let mut i = 1;
    for line in file_content.lines() {
        if i >= config.start_line && i <= config.end_line {
            result.push_str(&format!("{line}\n"));
        }

        i += 1;
    }

    Ok(result)
}

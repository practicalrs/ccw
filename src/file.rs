use crate::{Result, config::Config};
use std::{fs::read_to_string, sync::Arc};

pub fn read(config: &Arc<Config>) -> Result<String> {
    let mut result = String::new();
    let file_content = read_to_string(config.file.clone())?;

    if let (Some(start_line), Some(end_line)) = (config.start_line, config.end_line) {
        let mut i = 1;
        for line in file_content.lines() {
            if i >= start_line && i <= end_line {
                result.push_str(&format!("{line}\n"));
            }

            i += 1;
        }
    } else {
        result = file_content;
    };

    Ok(result)
}

use crate::{Result, config::Config};
use std::{fs::read_to_string, sync::Arc};
use walkdir::WalkDir;

pub fn read(config: &Arc<Config>, file: &str) -> Result<String> {
    let mut result = String::new();
    let file_content = read_to_string(file)?;

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
    }

    Ok(result)
}

pub fn read_files(config: &Arc<Config>) -> Result<Vec<(String, String)>> {
    let mut result = vec![];

    if let Some(dir) = &config.dir {
        for entry in WalkDir::new(dir) {
            let entry = entry?;
            let file = format!("{}", entry.path().display());
            if file.ends_with(".rs") {
                let content = read(config, &file)?;

                result.push((file.clone(), content));
            }
        }
    }

    if let Some(file) = &config.file {
        let content = read(config, file)?;

        result.push((file.clone(), content));
    }

    Ok(result)
}

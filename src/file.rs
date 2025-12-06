use crate::{Result, config::Config};
use std::{fs::read_to_string, path::Path, sync::Arc};
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

    let allowed_extensions = vec![
        "c", "h", // C
        "cpp", "hpp", "cc", "hh", "cxx", "hxx", // C++
        "cs",  // C#
        "go",  // Go
        "java", "js", // Java ;)
        "py", // Python
        "rs", // Rust
        "ts", // TypeScript
    ];

    if let Some(dir) = &config.dir {
        for entry in WalkDir::new(dir) {
            let entry = entry?;
            let path = entry.path();

            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                if allowed_extensions.contains(&ext) {
                    let file = format!("{}", path.display());
                    let content = read(config, &file)?;
                    result.push((file, content));
                }
            }
        }
    }

    if let Some(file) = &config.file {
        if let Some(ext) = Path::new(file).extension().and_then(|s| s.to_str()) {
            if allowed_extensions.contains(&ext) {
                let content = read(config, file)?;

                result.push((file.clone(), content));
            }
        }
    }

    Ok(result)
}

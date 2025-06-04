use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;

pub fn scan_naming(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let snake_case = Regex::new(r"^[a-z0-9_]+$")?;
    let mut stack = vec![path.to_path_buf()];

    while let Some(current) = stack.pop() {
        for entry in fs::read_dir(current)? {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name().into_string().unwrap_or_default();
            
            if !snake_case.is_match(&name) {
                println!("Non-snake_case name: {}", path.display());
            }

            if path.is_dir() {
                stack.push(path);
            }
        }
    }

    Ok(())
}

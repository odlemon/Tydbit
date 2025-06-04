use std::fs;
use std::path::Path;
use regex::Regex;

fn should_ignore(path: &Path) -> bool {
    let ignore_patterns = [
        "target",
        "node_modules",
        ".git",
        ".idea",
        ".vscode",
        "dist",
        "build",
    ];

    if let Some(path_str) = path.to_str() {
        for pattern in ignore_patterns {
            if path_str.contains(pattern) {
                return true;
            }
        }
    }

    false
}

pub fn scan_naming(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let snake_case = Regex::new(r"^[a-z0-9_]+$")?;
    let mut stack = vec![path.to_path_buf()];
    let mut found_issues = false;

    while let Some(current) = stack.pop() {
        if should_ignore(&current) {
            continue;
        }

        for entry in fs::read_dir(current)? {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name().into_string().unwrap_or_default();
            
            if !snake_case.is_match(&name) {
                if !found_issues {
                    println!("\n🔍 Found naming inconsistencies:");
                    found_issues = true;
                }
                println!("  📄 {}", path.display());
            }

            if path.is_dir() && !should_ignore(&path) {
                stack.push(path);
            }
        }
    }

    if !found_issues {
        println!("\n✨ No naming inconsistencies found!");
    }

    Ok(())
}

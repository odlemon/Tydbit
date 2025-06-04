mod analyzer;

use std::env;
use std::path::Path;
use analyzer::typos::TypoAnalyzer;
use std::fs;

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

fn is_text_file(path: &Path) -> bool {
    let text_extensions = [
        "txt", "md", "markdown", "rst", "adoc",
        "html", "htm", "css", "js", "ts",
        "jsx", "tsx", "json", "yaml", "yml",
        "toml", "ini", "cfg", "conf",
        "rs", "py", "java", "c", "cpp", "h", "hpp",
        "go", "rb", "php", "cs", "swift",
    ];

    if let Some(extension) = path.extension() {
        if let Some(ext_str) = extension.to_str() {
            return text_extensions.contains(&ext_str.to_lowercase().as_str());
        }
    }

    false
}

fn scan_directory(typo_analyzer: &TypoAnalyzer, dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut stack = vec![dir.to_path_buf()];
    let mut found_typos = false;

    while let Some(current) = stack.pop() {
        if should_ignore(&current) {
            continue;
        }

        for entry in fs::read_dir(current)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && is_text_file(&path) {
                match typo_analyzer.scan_file(&path) {
                    Ok(typos) => {
                        if !typos.is_empty() {
                            if !found_typos {
                                println!("\n🔍 Found potential typos:");
                                found_typos = true;
                            }
                            println!("\n📄 {}", path.display());
                            for (word, line) in typos {
                                println!("  Line {}: '{}'", line, word);
                            }
                        }
                    }
                    Err(e) => eprintln!("⚠️  Error scanning file {}: {}", path.display(), e)
                }
            } else if path.is_dir() {
                stack.push(path);
            }
        }
    }

    if !found_typos {
        println!("\n✨ No typos found in text files!");
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 {
        Path::new(&args[1])
    } else {
        Path::new(".")
    };

    println!("🔎 Analyzing path: {}", path.display());
    let typo_analyzer = TypoAnalyzer::new();

    if path.is_file() {
        if is_text_file(path) {
            match typo_analyzer.scan_file(path) {
                Ok(typos) => {
                    if typos.is_empty() {
                        println!("✨ No typos found!");
                    } else {
                        println!("\n🔍 Found potential typos:");
                        for (word, line) in typos {
                            println!("  Line {}: '{}'", line, word);
                        }
                    }
                }
                Err(e) => eprintln!("⚠️  Error scanning for typos: {}", e)
            }
        } else {
            println!("⚠️  Not a text file, skipping.");
        }
    } else if path.is_dir() {
        if let Err(e) = analyzer::naming::scan_naming(path) {
            eprintln!("⚠️  Error during naming scan: {}", e);
        }

        if let Err(e) = scan_directory(&typo_analyzer, path) {
            eprintln!("⚠️  Error during typo scan: {}", e);
        }
    } else {
        eprintln!("❌ Error: Path '{}' does not exist", path.display());
    }
}

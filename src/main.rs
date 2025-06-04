mod analyzer;

use std::env;
use std::path::Path;
use analyzer::typos::TypoAnalyzer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <PATH>", args[0]);
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);
    println!("Analyzing path: {}", path.display());

    if path.is_dir() {
        if let Err(e) = analyzer::naming::scan_naming(path) {
            eprintln!("Error during naming scan: {}", e);
        }
    }

    let typo_analyzer = TypoAnalyzer::new();
    
    if path.is_file() {
        match typo_analyzer.scan_file(path) {
            Ok(typos) => {
                for (word, line) in typos {
                    println!("Possible typo '{}' at line {}", word, line);
                }
            }
            Err(e) => eprintln!("Error scanning for typos: {}", e)
        }
    }
}

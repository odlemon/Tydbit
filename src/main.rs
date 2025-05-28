use std::env;
use std::path::Path;
mod analyzer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <PATH>", args[0]);
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);

    if let Err(e) = analyzer::naming::analyze_naming(path) {
        eprintln!("Naming analysis error: {}", e);
    }
}

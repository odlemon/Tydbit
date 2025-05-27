use std::env;
use std::path::Path;

mod analyzer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <PATH>", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];
    println!("Analyzing directory: {}", path);

    if let Err(e) = analyzer::scan_project(Path::new(path)) {
        eprintln!("Error scanning project: {}", e);
    }
}

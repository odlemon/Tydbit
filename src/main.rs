mod analyzer;

use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <PATH>", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];
    println!("Analyzing directory: {}", path);

    if let Err(e) = analyzer::naming::scan_naming(Path::new(path)) {
        eprintln!("Error during naming scan: {}", e);
    }
}

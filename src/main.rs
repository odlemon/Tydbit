mod cli;
mod analyzer;

use analyzer::typos;
use std::path::Path;

fn main() {
    let args = cli::get_args();
    let dir = Path::new(&args.path);

    typos::scan_for_typos(dir);
}

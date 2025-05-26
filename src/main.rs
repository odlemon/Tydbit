mod cli;

fn main() {
    let args = cli::get_args();

    println!("🔍 Scanning directory: {}", args.path.display());
}

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    
    pub path: PathBuf,
}

pub fn get_args() -> Args {
    Args::parse()
}

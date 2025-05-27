use std::fs;
use std::io;
use std::path::Path;

pub mod typos;

pub fn scan_project(path: &Path) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        println!("Found: {}", path.display());

        // Later, delegate to other analyzers like typos::check_file(&path)
    }

    Ok(())
}

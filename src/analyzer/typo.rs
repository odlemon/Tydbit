use std::fs;
use std::path::Path;

pub fn scan_for_typos(dir: &Path) {
    let entries = fs::read_dir(dir).unwrap();

    for entry in entries {
        let path = entry.unwrap().path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("txt") {
            let content = fs::read_to_string(&path).unwrap_or_default();
            if content.contains("teh") {
                println!("❌ Possible typo in {}: found 'teh'", path.display());
            }
        }
    }
}

use std::path::Path;
use std::fs;

pub fn analyze_naming(path: &Path) -> Result<(), String> {
    if !path.is_dir() {
        return Err("Path is not a directory".to_string());
    }

    for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        if name_str.contains(' ') {
            println!("Warning: '{}' contains spaces", name_str);
        }
    }

    Ok(())
}

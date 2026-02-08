use crate::Result;

pub fn list_files(path: &str) -> Result<Vec<String>> {
    let files: Vec<String> = std::fs::read_dir(path)
        .map_err(|e| format!("error while reading the directory: cause {e}"))?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect();

    if files.is_empty() {
        return Err("no files found in the directory".into());
    }

    Ok(files)
}

/// Recursively scan `src/` for filenames/directory names that appear in the ticket text.
/// Returns a sorted, deduplicated list of matching relative paths.
use walkdir::WalkDir;

pub fn find_code_references(description: &str, comments: &str) -> Vec<String> {
    let combined = format!("{} {}", description, comments).to_lowercase();

    let mut matches: Vec<String> = WalkDir::new("src")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name().to_string_lossy().to_lowercase();
            // Skip very short names to avoid noise
            if name.len() < 4 {
                return false;
            }
            // Also check stem (filename without extension)
            let stem = std::path::Path::new(&*name)
                .file_stem()
                .map(|s| s.to_string_lossy().to_lowercase())
                .unwrap_or_default();

            combined.contains(name.as_str())
                || (!stem.is_empty() && stem.len() >= 4 && combined.contains(stem.as_str()))
        })
        .map(|e| e.path().to_string_lossy().to_string())
        .collect();

    matches.sort();
    matches.dedup();
    matches
}

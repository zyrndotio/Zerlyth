use crate::model::ScanStatistics;
use anyhow::{Context, Result};
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ScannedFile {
    pub path: PathBuf,
    pub relative_path: String,
}

#[derive(Debug, Default)]
pub struct ScanResult {
    pub files: Vec<ScannedFile>,
    pub directories: Vec<String>,
    pub statistics: ScanStatistics,
    pub warnings: Vec<String>,
}

pub fn scan_repository(root: &Path) -> Result<ScanResult> {
    let root = root
        .canonicalize()
        .with_context(|| format!("cannot access repository path: {}", root.display()))?;
    if !root.is_dir() {
        anyhow::bail!("repository path is not a directory: {}", root.display());
    }

    let mut result = ScanResult::default();
    let walker = WalkBuilder::new(&root)
        .hidden(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .follow_links(false)
        .standard_filters(true)
        .build();

    for entry in walker {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path == root {
                    continue;
                }
                let relative_path = path
                    .strip_prefix(&root)
                    .unwrap_or(path)
                    .to_string_lossy()
                    .replace('\\', "/");
                if path.is_dir() {
                    result.directories.push(relative_path);
                    result.statistics.directories_scanned += 1;
                } else if path.is_file() {
                    let bytes = path.metadata().map(|m| m.len()).unwrap_or(0);
                    result.statistics.files_scanned += 1;
                    result.statistics.total_bytes += bytes;
                    result.files.push(ScannedFile {
                        path: path.to_path_buf(),
                        relative_path,
                    });
                }
            }
            Err(error) => {
                result.statistics.unreadable_entries += 1;
                result.warnings.push(error.to_string());
            }
        }
    }

    Ok(result)
}

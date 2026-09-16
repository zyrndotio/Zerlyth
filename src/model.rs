use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize)]
pub struct RepositoryReport {
    pub schema_version: &'static str,
    pub repository: RepositoryInfo,
    pub scan: ScanStatistics,
    pub project_types: Vec<ProjectTypeFinding>,
    pub important_files: Vec<ImportantFile>,
    pub directories: Vec<DirectorySummary>,
    pub code_summary: CodeSummary,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RepositoryInfo {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ScanStatistics {
    pub files_scanned: usize,
    pub directories_scanned: usize,
    pub ignored_entries: usize,
    pub unreadable_entries: usize,
    pub total_bytes: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectTypeFinding {
    pub name: String,
    pub evidence: Vec<String>,
    pub confidence: Confidence,
}

#[derive(Debug, Clone, Serialize)]
pub struct ImportantFile {
    pub path: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DirectorySummary {
    pub path: String,
    pub file_count: usize,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CodeSummary {
    pub source_files: usize,
    pub test_files: usize,
    pub source_lines: usize,
    pub comment_lines: usize,
    pub languages: Vec<LanguageSummary>,
    pub symbols: Vec<SymbolSummary>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LanguageSummary {
    pub language: String,
    pub files: usize,
    pub lines: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct SymbolSummary {
    pub name: String,
    pub kind: String,
    pub file: String,
    pub line: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
#[allow(dead_code)]
pub enum Confidence {
    High,
    Medium,
    Low,
}

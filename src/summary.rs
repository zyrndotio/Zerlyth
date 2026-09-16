use crate::model::{CodeSummary, LanguageSummary, SymbolSummary};
use crate::scanner::ScannedFile;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub fn summarize_code(files: &[ScannedFile], root: &Path) -> CodeSummary {
    let mut summary = CodeSummary::default();
    let mut languages: BTreeMap<String, (usize, usize)> = BTreeMap::new();

    for file in files {
        let Some(language) = language_for(&file.relative_path) else {
            continue;
        };
        let Ok(content) = fs::read_to_string(&file.path) else {
            continue;
        };
        let lines = content.lines().count();
        let comments = content
            .lines()
            .filter(|line| is_comment(line, language))
            .count();
        if is_test_path(&file.relative_path) {
            summary.test_files += 1;
        } else {
            summary.source_files += 1;
        }
        summary.source_lines += lines;
        summary.comment_lines += comments;
        let entry = languages.entry(language.to_string()).or_default();
        entry.0 += 1;
        entry.1 += lines;
        summary
            .symbols
            .extend(extract_symbols(&content, language, &file.relative_path));
    }

    summary.languages = languages
        .into_iter()
        .map(|(language, (files, lines))| LanguageSummary {
            language,
            files,
            lines,
        })
        .collect();
    let _ = root;
    summary
}

fn language_for(path: &str) -> Option<&'static str> {
    match Path::new(path).extension().and_then(|e| e.to_str()) {
        Some("rs") => Some("Rust"),
        Some("py") => Some("Python"),
        Some("js") | Some("jsx") | Some("mjs") => Some("JavaScript"),
        Some("ts") | Some("tsx") => Some("TypeScript"),
        Some("c") | Some("h") => Some("C"),
        Some("cc") | Some("cpp") | Some("cxx") | Some("hpp") => Some("C++"),
        Some("go") => Some("Go"),
        Some("java") | Some("kt") => Some("Java/Kotlin"),
        Some("cs") => Some("C#"),
        _ => None,
    }
}

fn is_test_path(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    lower.contains("/test")
        || lower.contains("tests/")
        || lower.contains("_test.")
        || lower.contains(".test.")
}

fn is_comment(line: &str, language: &str) -> bool {
    let trimmed = line.trim_start();
    if language == "Python" {
        trimmed.starts_with('#')
    } else {
        trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*')
    }
}

fn extract_symbols(content: &str, language: &str, file: &str) -> Vec<SymbolSummary> {
    let mut symbols = Vec::new();
    for (index, line) in content.lines().enumerate() {
        let trimmed = line.trim_start();
        let candidate = if language == "Rust" && trimmed.starts_with("pub fn ") {
            trimmed.trim_start_matches("pub fn ").split('(').next()
        } else if language == "Rust" && trimmed.starts_with("fn ") {
            trimmed.trim_start_matches("fn ").split('(').next()
        } else if (language == "Python" && trimmed.starts_with("def "))
            || (language == "JavaScript" && trimmed.starts_with("function "))
            || (language == "TypeScript" && trimmed.starts_with("function "))
        {
            trimmed
                .split_whitespace()
                .nth(1)
                .and_then(|value| value.split('(').next())
        } else if (language == "C++" || language == "C")
            && trimmed.contains('(')
            && trimmed.ends_with('{')
        {
            trimmed
                .split('(')
                .next()
                .and_then(|value| value.split_whitespace().last())
        } else {
            None
        };

        if let Some(name) = candidate.filter(|name| !name.is_empty() && *name != "unknown") {
            symbols.push(SymbolSummary {
                name: name.to_string(),
                kind: "function".to_string(),
                file: file.to_string(),
                line: index + 1,
            });
        }
    }
    symbols
}

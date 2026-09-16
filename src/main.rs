mod model;
mod parser;
mod scanner;
mod summary;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use model::{
    Confidence, DirectorySummary, ImportantFile, ProjectTypeFinding, RepositoryInfo,
    RepositoryReport,
};
use scanner::scan_repository;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
#[command(
    name = "zerlyth",
    version,
    about = "Find your way through any codebase."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Analyze a local repository.
    Analyze {
        #[arg(value_name = "PATH", default_value = ".")]
        path: PathBuf,
        #[arg(long, value_enum, default_value_t = OutputFormat::Terminal)]
        format: OutputFormat,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Print the Zerlyth version.
    Version,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputFormat {
    Terminal,
    Markdown,
    Json,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Version => println!("zerlyth {}", env!("CARGO_PKG_VERSION")),
        Command::Analyze {
            path,
            format,
            output,
        } => {
            let report = analyze(&path)?;
            let rendered = render(&report, format)?;
            if let Some(output_path) = output {
                fs::write(output_path, rendered)?;
            } else {
                print!("{rendered}");
            }
        }
    }
    Ok(())
}

fn analyze(path: &Path) -> Result<RepositoryReport> {
    let scan = scan_repository(path)?;
    let canonical_path = path.canonicalize()?;
    let project_types = detect_projects(&scan.files);
    let important_files =
        scan.files
            .iter()
            .filter_map(|file| {
                let kind = match file.relative_path.to_ascii_lowercase().as_str() {
                    "readme.md" => "readme",
                    "contributing.md" => "contribution guide",
                    "license" | "license.md" | "license.txt" => "license",
                    "cargo.toml" | "package.json" | "pyproject.toml" | "go.mod"
                    | "cmakelists.txt" => "project manifest",
                    _ => return None,
                };
                Some(ImportantFile {
                    path: file.relative_path.clone(),
                    kind: kind.to_string(),
                })
            })
            .collect();
    let directories = scan
        .directories
        .iter()
        .map(|path| DirectorySummary {
            path: path.clone(),
            file_count: scan
                .files
                .iter()
                .filter(|f| f.relative_path.starts_with(&format!("{path}/")))
                .count(),
        })
        .collect();
    let code_summary = summary::summarize_code(&scan.files, path);
    Ok(RepositoryReport {
        schema_version: "0.1",
        repository: RepositoryInfo {
            name: canonical_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("repository")
                .to_string(),
            path: canonical_path,
        },
        scan: scan.statistics,
        project_types,
        important_files,
        directories,
        code_summary,
        warnings: scan.warnings,
    })
}

fn detect_projects(files: &[scanner::ScannedFile]) -> Vec<ProjectTypeFinding> {
    let names: Vec<&str> = files.iter().map(|f| f.relative_path.as_str()).collect();
    let mut findings = Vec::new();
    let checks = [
        ("Rust", vec!["Cargo.toml"]),
        ("JavaScript/TypeScript", vec!["package.json"]),
        (
            "Python",
            vec!["pyproject.toml", "requirements.txt", "setup.py"],
        ),
        ("Go", vec!["go.mod"]),
        ("C/C++", vec!["CMakeLists.txt", "Makefile"]),
        (".NET", vec![".sln", ".slnx", ".csproj"]),
    ];
    for (name, markers) in checks {
        let evidence: Vec<String> = names
            .iter()
            .filter(|path| markers.iter().any(|marker| path.ends_with(marker)))
            .map(|path| (*path).to_string())
            .collect();
        if !evidence.is_empty() {
            findings.push(ProjectTypeFinding {
                name: name.to_string(),
                evidence,
                confidence: Confidence::High,
            });
        }
    }
    findings
}

fn render(report: &RepositoryReport, format: OutputFormat) -> Result<String> {
    Ok(match format {
        OutputFormat::Json => serde_json::to_string_pretty(report)? + "\n",
        OutputFormat::Markdown => format!("# Zerlyth Repository Report\n\n- **Repository:** {}\n- **Files scanned:** {}\n- **Directories scanned:** {}\n- **Source files:** {}\n- **Test files:** {}\n\n## Detected projects\n\n{}\n\n## Languages\n\n{}\n", report.repository.name, report.scan.files_scanned, report.scan.directories_scanned, report.code_summary.source_files, report.code_summary.test_files, report.project_types.iter().map(|p| format!("- {} ({:?})", p.name, p.confidence)).collect::<Vec<_>>().join("\n"), report.code_summary.languages.iter().map(|l| format!("- {}: {} files, {} lines", l.language, l.files, l.lines)).collect::<Vec<_>>().join("\n")),
        OutputFormat::Terminal => format!("Zerlyth Repository Report\n\nRepository: {}\nFiles scanned: {}\nDirectories scanned: {}\nSource files: {}\nTest files: {}\n\nDetected projects:\n{}\n\nLanguages:\n{}\n", report.repository.name, report.scan.files_scanned, report.scan.directories_scanned, report.code_summary.source_files, report.code_summary.test_files, report.project_types.iter().map(|p| format!("  - {}", p.name)).collect::<Vec<_>>().join("\n"), report.code_summary.languages.iter().map(|l| format!("  - {}: {} files, {} lines", l.language, l.files, l.lines)).collect::<Vec<_>>().join("\n")),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn detects_rust_manifest() {
        let files = vec![scanner::ScannedFile {
            path: PathBuf::from("Cargo.toml"),
            relative_path: "Cargo.toml".into(),
        }];
        assert_eq!(detect_projects(&files)[0].name, "Rust");
    }
}

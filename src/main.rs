use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};
use git2::Repository;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about = "Analyze a GitHub repository to understand its structure and requirements")]
struct Args {
    /// GitHub Repository URL
    #[arg(short, long)]
    url: String,
}

struct ProjectAnalysis {
    name: String,
    dependencies: Vec<String>,
    files: Vec<FileInfo>,
    purpose: String,
    contribution_notes: String,
}

struct FileInfo {
    path: PathBuf,
    role: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("{}", "🚀 Analyzing Repository...".cyan().bold());
    
    let temp_dir = TempDir::new().context("Failed to create temp directory")?;
    let repo_path = temp_dir.path();

    clone_repo(&args.url, repo_path)?;
    let analysis = analyze_project(repo_path)?;
    
    interactive_review(analysis);

    Ok(())
}

fn clone_repo(url: &str, path: &Path) -> Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner().template("{spinner:.green} {msg}")?);
    pb.set_message("Cloning repository...");

    Repository::clone(url, path).context("Failed to clone repository")?;
    
    pb.finish_with_message("Repository cloned successfully.");
    Ok(())
}

fn analyze_project(path: &Path) -> Result<ProjectAnalysis> {
    let mut dependencies = Vec::new();
    let mut files = Vec::new();
    let mut purpose = "Unknown project purpose.".to_string();
    let mut contribution_notes = "No specific contribution notes found.".to_string();

    // Identify project name
    let name = path.file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("Unknown Project")
        .to_string();

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let entry_path = entry.path();
        if entry_path.is_dir() { continue; }

        let filename = entry_path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        let relative_path = entry_path.strip_prefix(path).unwrap_or(entry_path).to_path_buf();

        // Dependency detection
        match filename {
            "package.json" => {
                dependencies.push("Node.js / NPM".to_string());
                files.push(FileInfo { path: relative_path, role: "Dependency Manifest (JS)".to_string() });
            },
            "Cargo.toml" => {
                dependencies.push("Rust / Cargo".to_string());
                files.push(FileInfo { path: relative_path, role: "Dependency Manifest (Rust)".to_string() });
            },
            "requirements.txt" | "pyproject.toml" => {
                dependencies.push("Python".to_string());
                files.push(FileInfo { path: relative_path, role: "Dependency Manifest (Python)".to_string() });
            },
            "go.mod" => {
                dependencies.push("Go".to_string());
                files.push(FileInfo { path: relative_path, role: "Dependency Manifest (Go)".to_string() });
            },
            "README.md" | "README" => {
                files.push(FileInfo { path: relative_path.clone(), role: "Project Documentation".to_string() });
                if let Ok(content) = fs::read_to_string(entry_path) {
                    purpose = extract_purpose(&content);
                }
            },
            _ => {
                let role = determine_role(filename);
                files.push(FileInfo { path: relative_path, role });
            }
        }
    }

    Ok(ProjectAnalysis {
        name,
        dependencies,
        files,
        purpose,
        contribution_notes,
    })
}

fn determine_role(filename: &str) -> String {
    if filename.ends_with(".md") { "Documentation".to_string() }
    else if filename.ends_with(".rs") { "Source Code (Rust)".to_string() }
    else if filename.ends_with(".py") { "Source Code (Python)".to_string() }
    else if filename.ends_with(".js") || filename.ends_with(".ts") { "Source Code (JS/TS)".to_string() }
    else if filename.ends_with(".cpp") || filename.ends_with(".h") { "Source Code (C++)".to_string() }
    else { "Project File".to_string() }
}

fn extract_purpose(content: &str) -> String {
    let lines: Vec<&str> = content.lines().take(10).collect();
    lines.first().unwrap_or(&"No description found").to_string()
}

fn interactive_review(analysis: ProjectAnalysis) {
    let options = vec![
        "Project Overview",
        "Required Software (Dependencies)",
        "File Structure & Roles",
        "Exit",
    ];

    loop {
        println!("\n{}", "--- Project Report ---".bold().underline());
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What would you like to review?")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => {
                println!("\nProject: {}", analysis.name.green().bold());
                println!("Purpose: {}", analysis.purpose);
            },
            1 => {
                println!("\n{}", "Required Software:".yellow().bold());
                if analysis.dependencies.is_empty() {
                    println!("No standard dependency files found.");
                } else {
                    for dep in &analysis.dependencies {
                        println!("- {}", dep);
                    }
                }
            },
            2 => {
                println!("\n{}", "File Mapping:".yellow().bold());
                for file in &analysis.files {
                    println!("{:<40} | {}", file.path.display().to_string().blue(), file.role);
                }
            },
            3 => break,
            _ => {}
        }
    }
}

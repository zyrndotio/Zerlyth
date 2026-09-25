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
#[command(author, version, about = "Professional Repository Insight Tool")]
struct Args {
    /// GitHub Repository URL
    #[arg(short, long)]
    url: String,
}

struct ProjectAnalysis {
    name: String,
    detailed_deps: Vec<String>,
    files: Vec<FileInfo>,
    purpose: String,
    entry_point: Option<PathBuf>,
}

struct FileInfo {
    path: PathBuf,
    role: String,
    importance: Importance,
}

#[derive(PartialEq, PartialOrd)]
enum Importance {
    Low,
    Medium,
    High,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("{}", "🛡️  Zerlyth v2.0 | Deep Analysis Mode".cyan().bold());
    
    // Extract project name from URL
    let project_name = args.url.split('/').last().unwrap_or("Unknown Project").replace(".git", "");
    
    let temp_dir = TempDir::new().context("Failed to create temp directory")?;
    let repo_path = temp_dir.path();

    clone_repo(&args.url, repo_path)?;
    let analysis = analyze_project(repo_path, &project_name)?;
    
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

fn analyze_project(path: &Path, name: &str) -> Result<ProjectAnalysis> {
    let mut detailed_deps = Vec::new();
    let mut files = Vec::new();
    let mut purpose = "No description found.".to_string();
    let mut entry_point = None;

    // 1. Deep Dependency Parsing
    if let Ok(cargo_content) = fs::read_to_string(path.join("Cargo.toml")) {
        for line in cargo_content.lines() {
            if line.trim().starts_with("[dependencies]") || line.trim().starts_with("[dev-dependencies]") {
                continue;
            }
            if line.contains('=') && line.trim().starts_with(|c: char| c.is_alphanumeric()) {
                let dep = line.split('=').next().unwrap_or("").trim().to_string();
                detailed_deps.push(format!("Rust Crate: {}", dep));
            }
        }
    }

    // 2. Structural Scan
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let entry_path = entry.path();
        if entry_path.is_dir() || entry_path.to_string_lossy().contains(".git") { continue; }

        let filename = entry_path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        let relative_path = entry_path.strip_prefix(path).unwrap_or(entry_path).to_path_buf();

        let (role, importance) = match filename {
            "README.md" | "README" => {
                if let Ok(content) = fs::read_to_string(entry_path) {
                    purpose = content.lines().next().unwrap_or("No description").to_string();
                }
                ("Project Documentation".to_string(), Importance::High)
            },
            "main.rs" => {
                entry_point = Some(relative_path.clone());
                ("Application Entry Point".to_string(), Importance::High)
            },
            "lib.rs" => ("Library Core Logic".to_string(), Importance::High),
            "Cargo.toml" => ("Build Configuration".to_string(), Importance::Medium),
            f if f.ends_with(".rs") => ("Source Code (Rust)".to_string(), Importance::Medium),
            f if f.ends_with(".md") => ("Documentation".to_string(), Importance::Low),
            _ => ("Project Asset/Config".to_string(), Importance::Low),
        };

        files.push(FileInfo { path: relative_path, role, importance });
    }

    // Sort files by importance so the most critical ones show up first
    files.sort_by(|a, b| b.importance.partial_cmp(&a.importance).unwrap());

    Ok(ProjectAnalysis {
        name: name.to_string(),
        detailed_deps,
        files,
        purpose,
        entry_point,
    })
}

fn interactive_review(analysis: ProjectAnalysis) {
    let options = vec![
        "Project Overview",
        "Deep Dependency List",
        "Core Structure & Entry Points",
        "Contribution Guide",
        "Exit",
    ];

    loop {
        println!("\n{}", "--- Zerlyth Analysis Report ---".bold().underline().cyan());
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a report section")
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
                println!("\n{}", "Detected Dependencies:".yellow().bold());
                if analysis.detailed_deps.is_empty() {
                    println!("No specific dependencies found.");
                } else {
                    for dep in &analysis.detailed_deps {
                        println!("- {}", dep);
                    }
                }
            },
            2 => {
                println!("\n{}", "Core File Mapping (Sorted by Importance):".yellow().bold());
                if let Some(ep) = &analysis.entry_point {
                    println!("🚀 Main Entry Point: {}", ep.display().to_string().green().bold());
                }
                println!("--------------------------------------------------");
                for file in &analysis.files {
                    let color_path = match file.importance {
                        Importance::High => file.path.display().to_string().red(),
                        Importance::Medium => file.path.display().to_string().yellow(),
                        Importance::Low => file.path.display().to_string().blue(),
                    };
                    println!("{:<40} | {}", color_path, file.role);
                }
            },
            3 => {
                println!("\n{}", "How to contribute to this project:".yellow().bold());
                if let Some(ep) = &analysis.entry_point {
                    println!("1. Start by reading the entry point: {}", ep.display().to_string().cyan());
                    println!("2. Look for logic in files marked as 'Core Logic' or 'Source Code'.");
                } else {
                    println!("1. Explore the 'src' folder to find the main logic.");
                }
                println!("3. Always check the README.md for specific build instructions.");
            },
            4 => break,
            _ => {}
        }
    }
}

#![allow(dead_code)]
//! Code parsing boundary for Zerlyth.
//!
//! The MVP uses conservative, evidence-based heuristics in `summary` so the
//! scanner remains lightweight and dependency-free. This module defines the
//! contract for adding real AST parsers without changing the report model.

use crate::model::SymbolSummary;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParserKind {
    Heuristic,
    TreeSitter,
}

pub trait CodeParser {
    fn kind(&self) -> ParserKind;
    fn supports(&self, path: &Path) -> bool;
    fn symbols(&self, source: &str, path: &Path) -> Vec<SymbolSummary>;
}

/// Selects the parsing strategy for a source file.
///
/// Tree-sitter parsers should be added behind feature flags after the report
/// contract and fixture suite stabilize. Parser failures must fall back to
/// heuristics and become warnings rather than aborting a repository scan.
pub fn parser_strategy(path: &Path) -> ParserKind {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("rs" | "py" | "js" | "jsx" | "ts" | "tsx" | "c" | "h" | "cpp" | "hpp") => {
            ParserKind::Heuristic
        }
        _ => ParserKind::Heuristic,
    }
}

# Zerlyth

> Find your way through any codebase.

Zerlyth is a privacy-first command-line tool for understanding unfamiliar repositories. It scans a local codebase, respects `.gitignore`, detects project technologies, summarizes source files, and generates structured reports for developers and contributors.

## Current status

Early development. The first vertical slice is implemented:

- safe local filesystem traversal using the `ignore` crate;
- `.gitignore`, global ignore, and standard build-output filtering;
- Rust, Python, JavaScript, TypeScript, C, C++, Go, Java/Kotlin, and C# language summaries;
- heuristic function-symbol extraction;
- project manifest detection;
- terminal, Markdown, and JSON report output;
- a parser abstraction ready for future Tree-sitter integration.

Zerlyth does not upload source code or execute detected build and test commands during analysis.

## Quick start

```bash
cargo run -- analyze .
cargo run -- analyze . --format markdown --output zerlyth-report.md
cargo run -- analyze . --format json --output zerlyth-report.json
cargo run -- version
```

## Example JSON report fields

```json
{
  "schema_version": "0.1",
  "repository": {},
  "scan": {},
  "project_types": [],
  "important_files": [],
  "directories": [],
  "code_summary": {},
  "warnings": []
}
```

The JSON schema is intentionally small and will evolve as the analyzer gains more detectors. Reports use evidence-based findings instead of claiming complete semantic understanding of a repository.

## Development

```bash
cargo check
cargo test
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

See [CONTRIBUTING.md](CONTRIBUTING.md), [SECURITY.md](SECURITY.md), and the project documentation for design boundaries and development expectations.

## License

Zerlyth is licensed under the MIT License.

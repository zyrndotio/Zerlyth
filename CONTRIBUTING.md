# Contributing to Zerlyth

Thank you for your interest in Zerlyth. The project is in early development, so small, focused contributions are especially valuable.

## Before starting

For a new feature or detector, open an issue first and describe the use case, evidence, and proposed scope. For a bug, include the operating system, Zerlyth version, command used, and a minimal repository fixture when possible.

## Development setup

Install the current stable Rust toolchain, then run:

```bash
cargo check
cargo test
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

## Design rules

Zerlyth's local analysis must not upload source code, execute detected commands, modify analyzed repositories, or access the network by default. Detectors should return structured findings with evidence paths rather than printing directly.

When a real repository exposes a bug, add a focused regression test or fixture. Keep pull requests small enough to review and explain the user-visible behavior in the pull request description.

## Pull requests

Pull requests should include:

- a concise explanation of the problem;
- the intended behavior;
- tests or fixture changes;
- documentation updates when the CLI or report schema changes;
- any known limitations.

By contributing, you agree that your contribution is provided under the MIT License.

# 🛡️ Zerlyth

Zerlyth is a powerful Rust-based CLI tool designed to help developers quickly onboard into unfamiliar codebases. Instead of spending hours digging through folders, Zerlyth clones a repository, scans its structure, identifies the required tech stack, and generates an interactive report for you to review.

## ✨ Features
- **Instant Analysis**: Clone any GitHub repository and get a high-level overview immediately.
- **Dependency Detection**: Automatically detects if a project uses Rust, Node.js, Python, Go, or other major ecosystems.
- **File Role Mapping**: Categorizes files (Source Code, Documentation, Configs) so you know where the "brains" of the project live.
- **Interactive Review**: A built-in menu system to navigate the generated report without leaving the terminal.
- **Cross-Platform**: Fully compatible with Windows, macOS, and Linux.

## 🛠️ Installation & Setup

### Prerequisites
Before installing Zerlyth, ensure you have the following installed on your system:
1. **Rust Toolchain**: Install via [rustup.rs](https://rustup.rs/).
2. **Git**: Required for cloning repositories. [Download Git](https://git-scm.com/downloads).
3. **C++ Build Tools** (Windows only): If you encounter linker errors, install the [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/).

### Building from Source
```bash
# Clone this repository
git clone https://github.com/your-username/zerlyth.git
cd zerlyth

# Build the project
cargo build --release
```

## 🚀 Usage
Run the tool by passing a GitHub repository URL:

```bash
cargo run -- --url https://github.com/owner/repo
```

Once the analysis is complete, use the arrow keys to navigate the report:
- **Project Overview**: Quick summary and purpose.
- **Required Software**: A list of languages and package managers needed to run the code.
- **File Structure**: A map of every file and its identified role.

## ⚖️ Licensing
This project is distributed under a **Custom Proprietary License**. 

**Key Restrictions:**
- **Credit**: Attribution to the original author is required.
- **Restrictions**: Commercial redistribution or modification for redistribution is prohibited without explicit permission.
- See the `LICENSE` file for full details.

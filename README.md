# Vesper Harness 🚀

Vesper Harness is a 5-Stage Core Engine for autonomous AI coding. It integrates and harmonizes multiple AI agents including Aider, SWE-agent, and Hermes Agent (v0.20.0), allowing them to collaborate within a unified pipeline.

## Features
- **5-Stage State Machine**: Idle ➡️ Analyze ➡️ Risk Scan ➡️ Plan ➡️ Execute ➡️ Verify.
- **Skill Injection**: Dynamically injects skills into the prompt/IPC based on the required tags (e.g. `frontend`, `execute`).
- **Auto-Rules**: Automatically matches and injects development conventions based on the target files (e.g., Svelte patterns, Security rules).
- **Hermes Agent Integration**: Supports Plugin SDK and Artifacts live preview within the sandbox.

## Prerequisites
- Rust 1.80+ (or 2024 edition supported)
- [Cargo](https://doc.rust-lang.org/cargo/)

## Installation

```bash
git clone https://github.com/example/vesper-harness.git
cd vesper-harness
cargo build --release
```

## Configuration

Vesper Harness uses environment variables to locate external skills and rules. You can configure them in your environment:

- `VESPER_RULES_DIR`: Path to the directory containing automatic rules (default: `./rules`)
- `VESPER_SKILLS_DIR`: Path to the directory containing skill markdowns (default: `./skills`)

Example for PowerShell:
```powershell
$env:VESPER_RULES_DIR = "C:\path\to\your\rules"
$env:VESPER_SKILLS_DIR = "C:\path\to\your\skills"
```

## Usage

Run the harness from your terminal:

```bash
cargo run
```

When the prompt `Vesper> ` appears, enter a task using the `/task` command:

```text
Vesper> /task Create a login page using Svelte
```

The engine will then process your request through the 5 stages and orchestrate the AI agents to complete the task.

## Testing Skills

You can run the skill compatibility tester to ensure all your markdown skills can be loaded by the engine:

```bash
cargo run --bin test_skills
```

## License
MIT License. See `LICENSE` for details.

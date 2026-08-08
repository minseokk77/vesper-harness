# Vesper Harness 🚀

Vesper Harness is a 5-Stage Core Engine for autonomous AI coding. It integrates and harmonizes multiple AI agents including Aider, SWE-agent, and Hermes Agent (v0.20.0), allowing them to collaborate within a unified pipeline.

## Features
- **Approval-gated State Machine**: Idle ➡️ Analyze ➡️ Risk Scan ➡️ Plan ➡️ Pending Approval ➡️ Execute ➡️ Verify.
- **Real Agent Streaming**: Runs a configured coding agent as a subprocess and streams stdout/stderr.
- **Crash Resume**: Persists every transition to `.vesper/state.json` and restores with `--resume`.
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
- `VESPER_AGENT_PROGRAM`: Coding-agent executable (default: `aider`)
- `VESPER_AGENT_ARGS_JSON`: JSON string array of agent arguments. Supports `{task}` and `{instruction}` placeholders.
- `VESPER_VERIFY_PROGRAM`: Verification executable (default: `cargo`)
- `VESPER_VERIFY_ARGS_JSON`: JSON string array of verification arguments (default: `["test","--all-targets"]`)

Example for PowerShell:
```powershell
$env:VESPER_RULES_DIR = "C:\path\to\your\rules"
$env:VESPER_SKILLS_DIR = "C:\path\to\your\skills"
```

To reuse a logged-in Codex account without an OpenAI API key:

```powershell
$env:VESPER_AGENT_PROGRAM = "codex"
$env:VESPER_AGENT_ARGS_JSON = '["exec","--sandbox","workspace-write","--color","never","--skip-git-repo-check","{instruction}"]'
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

At `PendingApproval`, enter `y` to execute, `n` to cancel, or text feedback to revise `plan.md` and remain paused. To restore an interrupted run:

```bash
cargo run -- --resume
```

For automation after reviewing the generated plan policy, `--yes` uses the plain stdout protocol without opening the TUI:

```bash
cargo run -- --yes "Refactor the parser and run its tests"
```

## Testing Skills

You can run the skill compatibility tester to ensure all your markdown skills can be loaded by the engine:

```bash
cargo run --bin test_skills
```

## License
MIT License. See `LICENSE` for details.

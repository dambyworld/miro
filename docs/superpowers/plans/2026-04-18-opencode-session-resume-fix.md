# OpenCode Session Resume Fix Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fix OpenCode session entry so Miro launches the selected session with `opencode --session <session_id>`, while preserving Codex and Claude Code behavior.

**Architecture:** Keep the existing provider abstraction. Correct the OpenCode command mapping in `ProviderKind`, explicitly expose the provider CLI value as `opencode`, and add focused regression tests around command construction and CLI parsing.

**Tech Stack:** Rust, clap `ValueEnum`, cargo test, existing Miro provider model.

---

## File Structure

| File | Responsibility | Planned Change |
|------|----------------|----------------|
| `src/model.rs` | Defines provider names and provider-specific resume commands | Add command mapping tests, change OpenCode command to `opencode --session <id>`, force clap value name to `opencode` |
| `src/cli.rs` | Parses CLI commands and provider filters | Add parsing tests for `--provider codex`, `--provider claude-code`, and `--provider opencode` |
| `src/provider/opencode.rs` | Reads and deletes OpenCode sessions from SQLite | Remove unused `DateTime` import warning |

## Task 1: Add Failing Tests For Provider Resume Commands

**Files:**
- Modify: `src/model.rs`

- [ ] **Step 1: Add command mapping tests**

Append this test module to the bottom of `src/model.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::ProviderKind;

    #[test]
    fn codex_resume_command_uses_codex_resume() {
        let command = ProviderKind::Codex.resume_command("session-123");

        assert_eq!(command.program, "codex");
        assert_eq!(command.args, vec!["resume", "session-123"]);
    }

    #[test]
    fn claude_code_resume_command_uses_resume_flag() {
        let command = ProviderKind::ClaudeCode.resume_command("session-123");

        assert_eq!(command.program, "claude");
        assert_eq!(command.args, vec!["--resume", "session-123"]);
    }

    #[test]
    fn opencode_resume_command_uses_session_flag() {
        let command = ProviderKind::OpenCode.resume_command("ses_abc123");

        assert_eq!(command.program, "opencode");
        assert_eq!(command.args, vec!["--session", "ses_abc123"]);
    }
}
```

- [ ] **Step 2: Run the new failing OpenCode command test**

Run:

```bash
cargo test model::tests::opencode_resume_command_uses_session_flag
```

Expected result:

```text
FAILED
assertion `left == right` failed
left: ["resume", "ses_abc123"]
right: ["--session", "ses_abc123"]
```

- [ ] **Step 3: Implement the OpenCode command fix**

In `src/model.rs`, change only the OpenCode branch in `resume_command()` from:

```rust
Self::OpenCode => CommandSpec::new("opencode", ["resume", session_id]),
```

to:

```rust
Self::OpenCode => CommandSpec::new("opencode", ["--session", session_id]),
```

- [ ] **Step 4: Run provider command tests**

Run:

```bash
cargo test model::tests::
```

Expected result:

```text
test result: ok
```

- [ ] **Step 5: Commit command mapping fix**

Run:

```bash
git add src/model.rs
git commit -m "fix: use opencode session flag for resume"
```

## Task 2: Add Failing Tests For Provider CLI Parsing

**Files:**
- Modify: `src/cli.rs`
- Modify: `src/model.rs`

- [ ] **Step 1: Add CLI provider parsing tests**

In `src/cli.rs`, update the test imports from:

```rust
use super::Cli;
use crate::theme::ThemeName;
```

to:

```rust
use super::{Cli, Commands};
use crate::model::ProviderKind;
use crate::theme::ThemeName;
```

Then append these tests inside the existing `#[cfg(test)] mod tests` block:

```rust
#[test]
fn parses_codex_provider_filter() {
    let cli = Cli::parse_from(["miro", "list", "--provider", "codex"]);

    let Some(Commands::List { provider, .. }) = cli.command else {
        panic!("expected list command");
    };
    assert_eq!(provider, Some(ProviderKind::Codex));
}

#[test]
fn parses_claude_code_provider_filter() {
    let cli = Cli::parse_from(["miro", "list", "--provider", "claude-code"]);

    let Some(Commands::List { provider, .. }) = cli.command else {
        panic!("expected list command");
    };
    assert_eq!(provider, Some(ProviderKind::ClaudeCode));
}

#[test]
fn parses_opencode_provider_filter() {
    let cli = Cli::parse_from(["miro", "list", "--provider", "opencode"]);

    let Some(Commands::List { provider, .. }) = cli.command else {
        panic!("expected list command");
    };
    assert_eq!(provider, Some(ProviderKind::OpenCode));
}
```

- [ ] **Step 2: Run the new failing opencode provider parsing test**

Run:

```bash
cargo test cli::tests::parses_opencode_provider_filter
```

Expected result:

```text
FAILED
invalid value 'opencode' for '--provider <PROVIDER>'
```

- [ ] **Step 3: Make the OpenCode clap value explicit**

In `src/model.rs`, change the enum from:

```rust
pub enum ProviderKind {
    Codex,
    ClaudeCode,
    OpenCode,
}
```

to:

```rust
pub enum ProviderKind {
    Codex,
    ClaudeCode,
    #[value(name = "opencode")]
    OpenCode,
}
```

This keeps `as_str()` and display output unchanged while making clap accept `opencode`.

- [ ] **Step 4: Run CLI provider parsing tests**

Run:

```bash
cargo test cli::tests::parses_codex_provider_filter
cargo test cli::tests::parses_claude_code_provider_filter
cargo test cli::tests::parses_opencode_provider_filter
```

Expected result:

```text
test result: ok
```

- [ ] **Step 5: Commit CLI provider parsing fix**

Run:

```bash
git add src/cli.rs src/model.rs
git commit -m "fix: accept opencode provider filter"
```

## Task 3: Remove OpenCode Provider Warning

**Files:**
- Modify: `src/provider/opencode.rs`

- [ ] **Step 1: Verify the current warning**

Run:

```bash
cargo test provider::opencode::tests::returns_empty_when_no_database
```

Expected warning before the fix:

```text
warning: unused import: `DateTime`
```

- [ ] **Step 2: Remove the unused import**

In `src/provider/opencode.rs`, change:

```rust
use chrono::{DateTime, TimeZone, Utc};
```

to:

```rust
use chrono::{TimeZone, Utc};
```

- [ ] **Step 3: Run the focused OpenCode provider test**

Run:

```bash
cargo test provider::opencode::tests::returns_empty_when_no_database
```

Expected result:

```text
test result: ok
```

The unused `DateTime` warning no longer appears.

- [ ] **Step 4: Commit warning cleanup**

Run:

```bash
git add src/provider/opencode.rs
git commit -m "chore: remove unused opencode import"
```

## Task 4: Full Regression Verification

**Files:**
- No code changes expected

- [ ] **Step 1: Run all automated tests**

Run:

```bash
cargo test
```

Expected result:

```text
test result: ok
```

No `unused import: DateTime` warning appears.

- [ ] **Step 2: Verify provider-specific list commands**

Run:

```bash
cargo run -- list --provider codex
```

Expected result:

```text
[codex] ...
```

Run:

```bash
cargo run -- list --provider claude-code
```

Expected result:

```text
[claude-code] ...
```

Run:

```bash
cargo run -- list --provider opencode
```

Expected result:

```text
[opencode] ...
```

If a provider has no sessions on the machine running the verification, success is still defined as command exit code `0` and no provider value parsing error.

- [ ] **Step 3: Verify OpenCode CLI session option exists**

Run:

```bash
opencode --help
```

Expected output includes:

```text
-s, --session      session id to continue
```

- [ ] **Step 4: Inspect final diff**

Run:

```bash
git status --short
git log --oneline -n 5
```

Expected result:

```text
# git status --short has no unstaged or staged changes
```

Recent commits include:

```text
fix: use opencode session flag for resume
fix: accept opencode provider filter
chore: remove unused opencode import
```

## Task 5: Manual TUI Verification

**Files:**
- No code changes expected

- [ ] **Step 1: Launch Miro**

Run:

```bash
cargo run
```

Expected result: Miro TUI opens.

- [ ] **Step 2: Select OpenCode provider**

Press `f` until the header shows:

```text
filter:opencode
```

- [ ] **Step 3: Enter an OpenCode session**

Select an OpenCode session and press `Enter`.

Expected result: Miro leaves its alternate screen and OpenCode opens the selected session instead of immediately returning to Miro with a resume failure.

- [ ] **Step 4: Return to Miro**

Exit OpenCode normally.

Expected result: Miro re-enters its main screen and shows:

```text
resumed session and returned to Miro
```

- [ ] **Step 5: Record manual verification result**

If manual verification passes, no commit is required. If it fails, capture the exact failing command and output, then revise the plan or implementation before committing further changes.

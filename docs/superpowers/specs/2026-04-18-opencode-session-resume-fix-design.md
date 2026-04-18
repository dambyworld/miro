# OpenCode Session Resume Fix Design

> Date: 2026-04-18  
> Status: Approved design, implementation pending
> Project: miro

## Context

Recent commits added OpenCode support to `miro`:

- `ab38f7a feat(model): add OpenCode variant to ProviderKind`
- `60c6df1 feat(provider): create OpenCodeProvider with SQLite support`
- `5493488 feat(provider): register OpenCodeProvider in build_providers`
- `c857fe6 feat: add OpenCode provider support with SQLite integration`
- `84fd90d docs: update README to mention opencode support`

The existing design document, `docs/opencode-support-design.md`, listed the OpenCode resume command as `opencode resume <session_id>` with a note that it needed confirmation. The implementation copied that assumption into `ProviderKind::resume_command()`.

Local verification against OpenCode `1.4.10` shows that `opencode resume` is not a valid subcommand. OpenCode resumes a session through the global session option:

```bash
opencode --session <session_id>
```

This explains the user-visible bug: selecting an OpenCode session from the TUI exits the Miro alternate screen, runs an invalid OpenCode command, then returns to Miro instead of entering the target session.

## Goals

1. Entering an OpenCode session from Miro launches the correct OpenCode session.
2. Codex and Claude Code session resume behavior remains unchanged.
3. The provider filter CLI accepts `opencode`, matching the displayed provider name.
4. Tests cover provider command construction and provider CLI parsing so this integration does not regress.

## Non-Goals

1. Do not redesign the TUI.
2. Do not change the OpenCode SQLite schema mapping except for removing incidental warnings if touched.
3. Do not add version-specific OpenCode compatibility layers.
4. Do not change Codex or Claude Code data providers beyond regression tests.

## Current Defects

### Invalid OpenCode Resume Command

Current command:

```bash
opencode resume <session_id>
```

Correct command for OpenCode `1.4.10`:

```bash
opencode --session <session_id>
```

### Provider CLI Value Mismatch

`ProviderKind::OpenCode` derives `clap::ValueEnum`. By default this exposes the provider value as `open-code`, while Miro displays the provider as `opencode`.

Observed behavior:

```bash
cargo run -- list --provider opencode
# invalid value 'opencode'; possible values include 'open-code'

cargo run -- list --provider open-code
# works
```

The CLI must accept `opencode` because that is the provider name shown in list output, README, and user-facing language.

### Missing Regression Coverage

Existing tests verify that OpenCode sessions can be read from a test SQLite database, but they do not verify:

- OpenCode resume command shape
- Codex resume command shape
- Claude Code resume command shape
- `--provider opencode` parsing

## Design

### Resume Command Mapping

Update `ProviderKind::resume_command()`:

```rust
Self::OpenCode => CommandSpec::new("opencode", ["--session", session_id]),
```

Keep existing mappings unchanged:

```rust
Self::Codex => CommandSpec::new("codex", ["resume", session_id]),
Self::ClaudeCode => CommandSpec::new("claude", ["--resume", session_id]),
```

`run_resume_command()` remains unchanged. It already runs the returned command in the selected session `cwd` when the directory exists, restores the terminal after the child process exits, and records errors in TUI status.

### Provider CLI Name

Make the OpenCode `ValueEnum` spelling explicit so `--provider opencode` is accepted. The implementation will use a `clap` rename annotation on the enum variant, while keeping serde/user display behavior unchanged.

Expected user-facing provider names after the fix:

- `codex`
- `claude-code`
- `opencode`

### TUI Provider Filter

Keep the current cycle order:

```text
all -> codex -> claude-code -> opencode -> all
```

The TUI already handles `ProviderKind::OpenCode` in the filter cycle and badge rendering, so no behavioral change is planned there.

### OpenCode Provider Cleanup

Remove the unused `DateTime` import from `src/provider/opencode.rs` if the file is touched during implementation. This is not a functional bug, but it currently creates a warning in `cargo test`.

## Error Handling

The existing error path in TUI remains:

1. Miro restores the terminal before launching the child process.
2. The provider command is executed.
3. If the command exits non-zero, Miro re-enters the alternate screen and shows `resume failed: ...`.
4. If the command succeeds, Miro refreshes sessions and shows `resumed session and returned to Miro`.

Changing OpenCode to a valid command makes the selected OpenCode session open instead of immediately failing and returning.

## Testing

### Automated Tests

Add tests for:

1. `ProviderKind::Codex.resume_command("id")` returns program `codex`, args `["resume", "id"]`.
2. `ProviderKind::ClaudeCode.resume_command("id")` returns program `claude`, args `["--resume", "id"]`.
3. `ProviderKind::OpenCode.resume_command("id")` returns program `opencode`, args `["--session", "id"]`.
4. CLI parsing accepts `miro list --provider opencode`.
5. CLI parsing still accepts `codex` and `claude-code`.

Run:

```bash
cargo test
```

### Manual Verification

Run provider list commands:

```bash
cargo run -- list --provider codex
cargo run -- list --provider claude-code
cargo run -- list --provider opencode
```

Verify OpenCode command support:

```bash
opencode --help
```

The help output must include `-s, --session` as a valid option. The implementation must not rely on `opencode resume`.

Manual TUI check when interactive verification is possible:

1. Run `cargo run`.
2. Use `f` until the provider filter is `opencode`.
3. Select an OpenCode session.
4. Press `Enter`.
5. Confirm OpenCode opens the selected session.
6. Exit OpenCode and confirm Miro returns to its main screen.

## Implementation Scope

Expected files:

- `src/model.rs`
- `src/cli.rs`
- `src/provider/opencode.rs`

No database migration, no TUI layout changes, and no release/version bump are part of this design.

## Success Criteria

1. OpenCode session resume uses `opencode --session <session_id>`.
2. Codex resume still uses `codex resume <session_id>`.
3. Claude Code resume still uses `claude --resume <session_id>`.
4. `cargo run -- list --provider opencode` works.
5. `cargo test` passes without the OpenCode unused import warning.

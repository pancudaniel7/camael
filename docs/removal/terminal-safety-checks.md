# Terminal Safety Checks

## Purpose

This document records the smallest existing automated checks that should stay green before and during any removal of Warp-related features from `pancudaniel7/camael`.

No new production terminal behavior was added or changed while preparing this document.

## Existing Terminal Test Surface

The repository already has clear terminal coverage, so no new smoke test code was added.

### Existing terminal-focused integration suites

- `crates/integration/tests/integration/shell_integration_tests.rs`
- `crates/integration/tests/integration/ui_tests.rs`
- `crates/integration/src/test.rs`
- `crates/integration/src/test/session_restoration.rs`
- `crates/integration/src/test/pane_restoration.rs`

### Existing terminal-focused unit/package tests

- `crates/warp_terminal`
- `app/src/terminal/*_tests.rs`
- `app/src/terminal/bootstrap_tests.rs`

## Recommended Minimal Smoke Suite

These existing integration tests are the smallest safe smoke suite I found that covers the requested terminal guarantees without adding new implementation or test scaffolding.

### 1. Terminal loads and command execution is wired

- Test: `test_single_command`
- Source: `crates/integration/src/test.rs`
- Why it matters:
  - waits for terminal bootstrap
  - verifies a terminal session can accept and execute a command

Run:

```bash
cargo run -p integration --bin integration -- test_single_command
```

### 2. Shell / session initialization path still exists

- Test: `test_terminal_announces_capabilities_to_shell`
- Source: `crates/integration/src/test.rs`
- Why it matters:
  - waits for bootstrap
  - verifies terminal startup environment is wired into the shell
  - checks `TERM`, `TERM_PROGRAM`, and `COLORTERM`

Run:

```bash
cargo run -p integration --bin integration -- test_terminal_announces_capabilities_to_shell
```

### 3. Session creation and close path is still wired

- Test: `test_add_and_close_session`
- Source: `crates/integration/src/test.rs`
- Why it matters:
  - opens another terminal session/tab
  - verifies the new session bootstraps
  - verifies close behavior for the session path

Run:

```bash
cargo run -p integration --bin integration -- test_add_and_close_session
```

### 4. Session persistence / restoration still exists

- Test: `test_session_restoration`
- Source: `crates/integration/src/test/session_restoration.rs`
- Why it matters:
  - restores a saved session snapshot
  - verifies restored terminal views exist and are bootstrapped
  - verifies restored tabs and terminal state are present

Run:

```bash
cargo run -p integration --bin integration -- test_session_restoration
```

## Additional Existing Checks Worth Keeping

These are not part of the smallest smoke suite above, but they are relevant terminal regressions detectors already present in the repo.

- `cargo nextest run -p warp_terminal`
  - package-level unit tests for terminal model, shell parsing, escape sequences, and grid behavior
- `test_restore_single_closed_pane`
  - pane restoration workflow
- `test_websocket_begins_on_startup`
  - startup websocket path for session/shared/cloud behavior
- `test_remote_server_connect_bash`
  - remote terminal connection path
- `test_tmux_ssh_into_bash`
  - ssh/tmux bootstrap path

## How To Run The Existing Terminal Checks

### Minimal smoke suite

```bash
cargo run -p integration --bin integration -- test_single_command
cargo run -p integration --bin integration -- test_terminal_announces_capabilities_to_shell
cargo run -p integration --bin integration -- test_add_and_close_session
cargo run -p integration --bin integration -- test_session_restoration
```

### Terminal unit/package tests

```bash
cargo nextest run -p warp_terminal
```

### Larger existing terminal integration surface

Examples:

```bash
cargo nextest run --no-fail-fast --workspace --exclude command-signatures-v2
cargo run -p integration --bin integration -- test_restore_single_closed_pane
cargo run -p integration --bin integration -- test_websocket_begins_on_startup
```

## Verification Run Performed

The following commands were run while preparing this document.

### Build

```bash
cargo build -p warp --bin warp-oss --features gui
```

Result:

- PASS

### Typecheck

```bash
cargo check -p warp --bin warp-oss --features gui
```

Result:

- PASS

### Formatting / lint available in current stack

```bash
cargo fmt -- --check
```

Result:

- PASS

```bash
cargo clippy --workspace --exclude warp_completer --all-targets --tests -- -D warnings
```

Result:

- FAIL, but unrelated to terminal behavior
- existing repo/tooling issue in `crates/command-signatures-v2`
- failure cause: build script expects Corepack-managed `yarn@4.0.1`, but environment resolved global `yarn 1.22.22`

### Unit tests

```bash
cargo nextest run -p warp_terminal
```

Result:

- PASS
- summary: `84 passed, 2 skipped`

### Focused terminal smoke tests

```bash
cargo run -p integration --bin integration -- test_single_command
cargo run -p integration --bin integration -- test_terminal_announces_capabilities_to_shell
cargo run -p integration --bin integration -- test_add_and_close_session
cargo run -p integration --bin integration -- test_session_restoration
```

Result:

- PASS
- all four completed successfully

## What These Checks Prove

If this smoke suite breaks after feature removal work, it is strong evidence that core terminal behavior was damaged:

- `test_single_command`
  - catches loss of terminal bootstrap or command execution wiring
- `test_terminal_announces_capabilities_to_shell`
  - catches broken shell initialization/bootstrap environment wiring
- `test_add_and_close_session`
  - catches broken session creation and lifecycle wiring
- `test_session_restoration`
  - catches broken persistence / restoration support
- `cargo nextest run -p warp_terminal`
  - catches lower-level terminal model, shell parsing, escape sequence, and grid regressions

## Current Recommendation

Before removing Warp-related functionality, keep the following as the required terminal gate:

```bash
cargo build -p warp --bin warp-oss --features gui
cargo check -p warp --bin warp-oss --features gui
cargo nextest run -p warp_terminal
cargo run -p integration --bin integration -- test_single_command
cargo run -p integration --bin integration -- test_terminal_announces_capabilities_to_shell
cargo run -p integration --bin integration -- test_add_and_close_session
cargo run -p integration --bin integration -- test_session_restoration
```

And treat the current Clippy failure as a separate pre-existing JS/Corepack environment issue, not as a terminal regression signal.

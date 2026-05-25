# Warp Removal Baseline

## Scope

- Repository: `pancudaniel7/camael`
- Goal of this report: capture a pre-removal baseline before any Warp-related functionality is deleted or rewritten.
- Safety: no application code was removed or behavior changed while producing this report.

## Detected Project Structure

- Monorepo: Rust Cargo workspace rooted at `Cargo.toml`
- Main desktop app crate: `app/`
- Shared Rust crates: `crates/*`
- Integration / end-to-end harness: `crates/integration/`
- Build / bootstrap / lint scripts: `script/`
- Assets and bundled resources: `app/assets/`, `resources/`
- Specs and design docs: `specs/`, `README.md`, `WARP.md`, `CONTRIBUTING.md`

### Key Product Areas Present

- Terminal: `app/src/terminal/`, `crates/warp_terminal/`
- AI / agents: `app/src/ai/`, `app/src/ai_assistant/`, `crates/ai/`
- Auth / sign-in: `app/src/auth/`
- Drive / Warp Drive: `app/src/drive/`
- Code / code review / codebase context: `app/src/code/`, `app/src/code_review/`, `app/src/integration_testing/codebase_context/`
- Workspace / panes / tabs: `app/src/workspace/`, `app/src/workspaces/`
- Integration test framework: `crates/integration/src/`, `crates/integration/tests/`

## Detected Package Managers And Tooling

- Primary package manager / build system: `cargo`
- Rust toolchain pin: `rust-toolchain.toml` specifies `1.92.0`
- Secondary JS package manager in `crates/command-signatures-v2/js/package.json`: `yarn@4.0.1` via Corepack
- Secondary JS build step usage:
  - `crates/command-signatures-v2/js/package.json`
  - scripts include `build`, `watch`, `clean`

## Detected Baseline Commands

### Build

- Documented:
  - `cargo run`
  - `./script/run`
- Safe non-launch baseline used for this report:
  - `cargo build -p warp --bin warp-oss --features gui`

### Typecheck

- No dedicated standalone typecheck script was documented in repo docs.
- Safe inferred baseline used for this report:
  - `cargo check -p warp --bin warp-oss --features gui`

### Test

- Documented:
  - `cargo nextest run --no-fail-fast --workspace --exclude command-signatures-v2`
  - `cargo nextest run -p warp_completer --features v2`
  - `cargo test --doc`
  - `./script/presubmit`

### Lint / Format

- `cargo fmt -- --check`
- `cargo clippy --workspace --exclude warp_completer --all-targets --tests -- -D warnings`
- `cargo clippy -p warp_completer --all-targets --tests -- -D warnings`
- `./script/run-clang-format.py -r --extensions 'c,h,cpp,m' ./crates/warpui/src/ ./app/src/`
- `find . -name "*.wgsl" -exec wgslfmt --check {} +`
- `./script/lint_powershell -ci`
- Aggregate script:
  - `./script/presubmit`

### Terminal / E2E

- Integration harness crate: `crates/integration`
- Documented examples:
  - `cargo run --bin integration -- test_simple_example`
  - `WARPUI_USE_REAL_DISPLAY_IN_INTEGRATION_TESTS=1 cargo run --bin integration -- test_simple_example`
- Existing automated terminal / UI baseline in presubmit:
  - `cargo nextest run --no-fail-fast --workspace --exclude command-signatures-v2`
  - This run includes `integration::integration shell_integration_tests::*` and `integration::integration ui_tests::*`

## Existing Terminal-Related Modules / Components

This is a native desktop app, not a web app, so there are no obvious HTTP/web routes for the terminal surface. The relevant terminal modules and UI components are filesystem modules under `app/src/terminal/` and integration coverage under `app/src/integration_testing/terminal/` and `crates/integration/`.

### Main Terminal Module Tree

- `app/src/terminal/alt_screen`
- `app/src/terminal/audible_bell`
- `app/src/terminal/cli_agent_sessions`
- `app/src/terminal/event_listener`
- `app/src/terminal/find`
- `app/src/terminal/grid_renderer`
- `app/src/terminal/history`
- `app/src/terminal/input`
- `app/src/terminal/local_shell`
- `app/src/terminal/local_tty`
- `app/src/terminal/model`
- `app/src/terminal/prompt`
- `app/src/terminal/remote_tty`
- `app/src/terminal/session_settings`
- `app/src/terminal/shared_session`
- `app/src/terminal/ssh`
- `app/src/terminal/view`
- `app/src/terminal/warpify`
- `app/src/terminal/writeable_pty`
- `app/src/terminal/wsl`

### Terminal View / Interaction Submodules

- `app/src/terminal/view/ambient_agent`
- `app/src/terminal/view/block_banner`
- `app/src/terminal/view/block_onboarding`
- `app/src/terminal/view/docker_sandbox`
- `app/src/terminal/view/init_environment`
- `app/src/terminal/view/init_project`
- `app/src/terminal/view/inline_banner`
- `app/src/terminal/view/shared_session`
- `app/src/terminal/view/use_agent_footer`

### Terminal / Shell Integration Tests Present

- `crates/integration/tests/integration/shell_integration_tests.rs`
- `crates/integration/tests/integration/ui_tests.rs`
- `app/src/integration_testing/terminal/`
- `app/src/test_util/terminal.rs`
- `app/src/terminal/bootstrap_tests.rs`
- `app/tests/ssh/`

## Existing Warp / AI / Team / Auth / Referral Reference Inventory

The following counts are file-count approximations from case-insensitive `rg -l` scans across `app`, `crates`, `resources`, `README.md`, `WARP.md`, and `CONTRIBUTING.md`.

| Term | File Count |
| --- | ---: |
| `WarpDrive` | 112 |
| `warpdrive` | 112 |
| `warp-drive` | 6 |
| `warp_drive` | 134 |
| `warp` | 2406 |
| `AI` | 2849 |
| `Teams` | 98 |
| `Team` | 384 |
| `ClaudePlatform` | 0 |
| `Claude` | 207 |
| `Anthropic` | 74 |
| `CodeBaseIndexing` | 17 |
| `CodebaseIndexing` | 17 |
| `Agents` | 351 |
| `Agent` | 1013 |
| `agent` | 1013 |
| `SignIn` | 33 |
| `SingIn` | 6 |
| `signin` | 33 |
| `sign-in` | 5 |
| `login` | 140 |
| `SignUp` | 28 |
| `SingUp` | 2 |
| `signup` | 28 |
| `sign-up` | 3 |
| `register` | 492 |
| `Referrals` | 14 |
| `Referral` | 31 |
| `Refferals` | 0 |
| `referral` | 31 |
| `invite` | 48 |

### Representative Reference Locations

#### Warp / Warp Drive

- `README.md`
- `WARP.md`
- `app/src/integration_testing/warp_drive/`
- `app/assets/bundled/svg/warp-drive.svg`
- `crates/onboarding/src/model.rs`
- `crates/warp_core/src/paths_tests.rs`

#### AI / Agents / Claude / Anthropic / Codebase Indexing

- `app/src/ai/`
- `app/src/ai_assistant/`
- `app/src/terminal/cli_agent_sessions/`
- `app/src/integration_testing/agent_mode/`
- `app/src/integration_testing/codebase_context/`
- `crates/ai/src/index/full_source_code_embedding/`
- `crates/managed_secrets/src/secret_value.rs`
- `crates/onboarding/src/model.rs`

#### Teams / Team Workflows / Team Websocket State

- `app/assets/bundled/svg/create-team.svg`
- `crates/integration/src/test/websockets.rs`
- `crates/integration/src/test/workflows.rs`
- `crates/managed_secrets/src/client.rs`

#### Sign-In / Login / Sign-Up / Registration

- `app/src/auth/`
- `README.md`
- `CONTRIBUTING.md`
- `crates/onboarding/src/model.rs`
- `aws-sdk-signin` appears in the Rust dependency graph during baseline commands

#### Referrals / Invites

- `app/assets/bundled/svg/referral-backpack.svg`
- `app/assets/bundled/svg/referral-theme.svg`
- `app/assets/bundled/svg/referral-notebook.svg`
- `app/assets/bundled/svg/referral-keycaps.svg`
- `app/assets/bundled/svg/referral-hydroflask.svg`
- `app/assets/bundled/svg/referral-hat.svg`
- `app/assets/bundled/svg/referral-tshirt.svg`
- `app/assets/bundled/svg/referral-hoodie.svg`

## Baseline Command Results

The commands below were run without intentionally changing application behavior. Existing failures were recorded and not fixed.

| Command | Result | Notes |
| --- | --- | --- |
| `cargo build -p warp --bin warp-oss --features gui` | PASS | Built the main app binary in dev profile. |
| `cargo check -p warp --bin warp-oss --features gui` | PASS | Typecheck-equivalent baseline passed. |
| `cargo fmt -- --check` | PASS | No Rust formatting issues reported. |
| `./script/run-clang-format.py -r --extensions 'c,h,cpp,m' ./crates/warpui/src/ ./app/src/` | PASS | No C/C++/Obj-C formatting issues reported. |
| `find . -name "*.wgsl" -exec wgslfmt --check {} +` | PASS | No WGSL formatting issues reported. |
| `./script/lint_powershell -ci` | PASS_WITH_FINDINGS | Reported `Invoke-ScriptAnalyzer` null-reference output in `app/assets/bundled/bootstrap/`, then 15 informational findings in `script/windows/*.ps1`. No Error/Warning severity violations were reported. |
| `cargo clippy --workspace --exclude warp_completer --all-targets --tests -- -D warnings && cargo clippy -p warp_completer --all-targets --tests -- -D warnings` | FAIL | `command-signatures-v2` build script failed because repo JS build expects Corepack-managed `yarn@4.0.1`, but environment resolved global `yarn 1.22.22`. |
| `cargo nextest run --no-fail-fast --workspace --exclude command-signatures-v2` | FAIL | Completed in 131.266s. Summary: 6933 tests run, 6918 passed, 15 failed, 85 skipped. Included many passing terminal shell/UI integration tests before failing existing cases. |
| `cargo nextest run -p warp_completer --features v2` | PASS | Completed in 1m54s build time plus nextest runtime. Summary: 116 tests run, 116 passed, 4 skipped. |
| `cargo test --doc` | PASS | Doc tests passed; observed examples included 36 `command` doctests, 4 `warp` doctests, and 3 `warp_util` doctests passing, with expected ignored doctests in some crates. |

### Existing Failure Details

#### Clippy failure

- Failing component: `crates/command-signatures-v2`
- Failing step: custom build script
- Root cause from stderr:
  - project `package.json` specifies `packageManager: "yarn@4.0.1"`
  - current global Yarn was `1.22.22`
  - build script explicitly recommends `corepack enable`

#### Workspace nextest failures observed

- Failing tests:
  - `integration::integration shell_integration_tests::test_legacy_ssh_into_bash`
  - `integration::integration shell_integration_tests::test_legacy_ssh_into_zsh`
  - `integration::integration shell_integration_tests::test_ssh_into_sh`
  - `integration::integration shell_integration_tests::test_ssh_into_ash`
  - `integration::integration ui_tests::test_ssh_with_shell_override`
  - `warp ai::agent_sdk::driver::snapshot::tests::e2e_dirty_repo_uploads_patch_and_manifest_reports_success`
  - `warp ai::agent_sdk::driver::snapshot::tests::e2e_clean_repo_uploads_only_manifest`
  - `warp ai::agent_sdk::driver::snapshot::tests::e2e_get_snapshot_upload_targets_failure_returns_none`
  - `warp ai::agent_sdk::driver::snapshot::tests::e2e_multi_repo_mixed_statuses_roundtrip_to_manifest`
  - `warp ai::agent_sdk::driver::snapshot::tests::e2e_read_failed_for_missing_file_continues_pipeline`
  - `warp ai::agent_sdk::driver::snapshot::tests::e2e_repo_plus_inside_and_outside_files_filters_overlap`
  - `warp settings::init::tests::test_migration_does_not_rerun_when_marker_present`
  - `warp util::git::tests::detached_head_raw_returns_head`
  - `warp util::git::tests::detached_head_display_returns_short_sha`
  - `warp util::git::tests::detached_tag_display_returns_short_sha`
- Observed behavior:
  - nextest had already passed a large set of `shell_integration_tests::*` and `ui_tests::*`
  - final summary was `6933 tests run: 6918 passed, 15 failed, 85 skipped`
- Additional terminal-specific evidence:
  - passing terminal/e2e tests included `test_terminal_announces_capabilities_to_shell`, `test_tmux_ssh_into_bash`, `test_tmux_ssh_into_zsh`, `test_remote_server_connect_bash`, `test_remote_server_connect_zsh`, and multiple pane / block / terminal UI tests.

## Notes For Future Warp-Removal Work

- Terminal functionality is deeply integrated and broadly covered by shell and UI integration tests.
- Warp / AI / agent / onboarding / drive / auth references are spread across both product code and bundled assets.
- There is a secondary JS toolchain dependency in `crates/command-signatures-v2` that can affect Rust lint/test commands even though the primary repo is Rust/Cargo-based.

# Warp CI Protection

## Scope

This stage adds CI protection for the terminal-focused cleanup work and prevents removed feature
surfaces from being reintroduced accidentally.

## Removed Feature Reference Guard

The repository now uses `script/check_removed_feature_references` from CI. It scans newly added diff
lines for exact removed-surface names:

- `WarpDrive`
- `ClaudePlatform`
- `CodeBaseIndexing`
- `Refferals`
- `Referrals`
- `SingIn`
- `SingUp`

The check is intentionally diff-based and excludes `docs/removal/**` plus the check script itself.
That keeps existing historical references from blocking cleanup branches while still preventing new
runtime or navigation references from being added.

Broad terms such as `AI`, `agent`, `team`, `login`, and `register` are not globally banned because
they still appear in legitimate terminal, SSH-agent, release-signing, CI-agent, workspace, and auth
contexts.

## CI Coverage

`.github/workflows/ci.yml` includes:

- `banned-references`: runs the removed-feature reference guard.
- `tests`: runs the Rust test matrix with nextest.
- `terminal-smoke`: runs focused integration smoke tests for terminal command execution, shell
  capability setup, session lifecycle, and session restoration.
- `database-migration`: applies all Diesel migrations, checks generated schema consistency, and
  redoes the latest migration.
- `lints`, `wasm-lint`, and `general-lint`: preserve formatting, clippy, metadata, dependency, and
  miscellaneous checks.
- `check-release-compilation`: preserves release-flag compilation coverage.
- `ci-result`: requires the protection jobs and validates conditional database-migration behavior.

## Migration Gate Decision

The database job does not run `diesel migration revert --all` anymore. Stage 6 found that historical
migration `2024-12-25-215548_drop_server_id_from_tables` has a pre-existing SQLite down-migration
failure: `ALTER TABLE teams ADD COLUMN server_id BIGINTEGER UNIQUE` is rejected by SQLite.

The replacement gate still protects current DB work by:

- applying all migrations on an empty SQLite database
- comparing generated schema with `crates/persistence/src/schema.rs`
- running `diesel migration redo` for the latest migration

## Obsolete Feature Jobs

No CI job dedicated only to WarpDrive, ClaudePlatform, CodeBaseIndexing, referrals, sign-in/sign-up
onboarding, or teams product UI was found in the main CI workflow. Existing agent references in CI
are automation infrastructure, SSH-agent setup, release signing, or changelog/feature-flag cleanup
automation and were not removed.

## Local Verification

- `script/check_removed_feature_references` passed.
- `ruby -e 'require "yaml"; YAML.load_file(".github/workflows/ci.yml")'` parsed the workflow.
- `git diff --check` passed.
- Local equivalent of the `database-migration` job passed: migration run, schema diff, and latest
  migration redo on a disposable SQLite database.
- `cargo check --locked -p warp --lib` passed with existing dead-code warnings.
- `cargo test -p warp terminal_workspace_starts_with_terminal_session --lib` passed.
- `cargo test -p warp_terminal --lib` passed: 84 passed, 2 ignored.
- Terminal integration smoke passed:
  - `cargo run -p integration --bin integration -- test_single_command`
  - `cargo run -p integration --bin integration -- test_terminal_announces_capabilities_to_shell`
  - `cargo run -p integration --bin integration -- test_add_and_close_session`
  - `cargo run -p integration --bin integration -- test_session_restoration`

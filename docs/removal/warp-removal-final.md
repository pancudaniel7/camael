# Warp Removal Final Cleanup

Status: Stage 8 destructive database cleanup completed for proven feature-owned tables.

## What Was Removed

Runtime database reads and writes were removed or made unreachable for these feature-owned objects:

- `agent_conversations`
- `agent_tasks`
- `ai_queries`
- `ai_memory_panes`
- `ai_document_panes`
- `ambient_agent_panes`
- `team_settings`
- `team_members`

The destructive migration `2026-05-29-120000_drop_removed_warp_feature_tables` drops those tables
and the associated agent-conversation update triggers. Diesel schema/model definitions for those
tables were removed.

## What Was Intentionally Kept

- Terminal tables and terminal pane persistence.
- PTY, shell, WebSocket, session create/reconnect, and command execution code.
- Shared auth/session/user/workspace data.
- Shared `teams`, `workspaces`, and `workspace_teams` tables.
- Risky embedded columns documented in `docs/removal/warp-db-inventory.md`, including terminal,
  block, command, notebook, window, and team columns that need separate proof before dropping.
- Terminal smoke tests and CI checks added in prior stages.

## Rollback

Rollback is the down migration for `2026-05-29-120000_drop_removed_warp_feature_tables`. It
recreates the dropped feature-owned tables and agent-conversation triggers, but it does not restore
dropped data. Operational rollback should restore the database from backup before rolling the app
back to a version that reads these tables.

## Terminal Verification Evidence

Verification performed during Stage 8:

- `diesel migration run --database-url /tmp/camael-stage8.sqlite --migration-dir crates/persistence/migrations --locked-schema`
- `diesel migration redo --database-url /tmp/camael-stage8.sqlite --migration-dir crates/persistence/migrations --locked-schema`
- `cargo check --locked -p warp --lib`
- `cargo test -p warp terminal_workspace_starts_with_terminal_session --lib`
- `cargo test -p warp_terminal --lib`
- `cargo run -p integration --bin integration -- test_single_command`

## Remaining Risks

- Deleted-surface runtime code names still exist outside the database layer and should be cleaned in
  follow-up stages if the product code itself must be fully removed.
- Embedded columns classified as risky in the DB inventory were intentionally not dropped.

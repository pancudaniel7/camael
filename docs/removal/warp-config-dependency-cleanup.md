# Warp Config And Dependency Cleanup

## Scope

This stage audited config, env-var usage, deployment manifests, and dependency declarations for
Warp-related surfaces that were previously targeted for removal.

## Repository Findings

- The repository does not currently contain checked-in `.env` examples, compose files, Helm charts,
  Kubernetes manifests, or Terraform configuration that could be cleaned up for these surfaces.
- The workspace is primarily Rust. The only lockfiles/manifests present are `Cargo.toml` /
  `Cargo.lock` plus a small number of JS package manifests used for schema/tooling.
- No feature-owned dependency package named for Anthropic, OpenAI, Claude, WarpDrive, referrals,
  or vector DB providers is declared in the Rust workspace manifests.
- No CI, Docker, README, or deployment config requires deleted feature secrets such as
  `ANTHROPIC_*`, `CLAUDE_*`, `OPENAI_*`, `CODEBASE_INDEXING_*`, `WARPDRIVE_*`, or `REFERRAL_*`.
- `docker/agent-dev/Dockerfile` contains optional CLI-agent installs for Claude Code and Codex
  behind `INSTALL_CODING_AGENTS=false`. Those installs were kept because terminal CLI-agent support
  still exists and the Docker path is opt-in tooling, not a required deleted-feature dependency.

## Safe Removals Performed

- Removed dead referral GraphQL client modules that became unused after the app-side referral API
  client was deleted:
  - `crates/graphql/src/api/queries/get_referral_info.rs`
  - `crates/graphql/src/api/mutations/send_referral_invite_emails.rs`
- Removed their module exports from:
  - `crates/graphql/src/api/queries/mod.rs`
  - `crates/graphql/src/api/mutations/mod.rs`

## Kept Live Env Var Usage

These env vars are still read by live terminal / CLI-agent code and were not removed in this stage:

- `ANTHROPIC_API_KEY`
- `ANTHROPIC_MODEL`
- `CLAUDE_CONFIG_DIR`
- `CLAUDE_HOME`
- `CLAUDE_CODE_USE_BEDROCK`
- `OPENAI_API_KEY`
- `OPENAI_BASE_URL`

Current live readers are in active app code under:

- `app/src/ai/agent_sdk/driver.rs`
- `app/src/ai/agent_sdk/driver/harness/claude_code.rs`
- `app/src/ai/agent_sdk/driver/harness/claude_transcript.rs`
- `app/src/ai/agent_sdk/driver/harness/codex.rs`
- `app/src/terminal/cli_agent_sessions/plugin_manager/claude.rs`
- `app/src/pane_group/pane/local_harness_launch.rs`

Because those terminal-adjacent agent, harness, and managed-secret code paths still exist in-tree,
removing their env vars, docs, optional Docker installs, or build features would be unsafe in this
stage.

## Remaining Feature Flags Or Build Features Still In Tree

These are still present and tied to live code paths, so they were not removed here:

- `remote_codebase_indexing`
- `local_claude_codex_child_harnesses`
- `team_workflows`
- `team_api_keys`

## Verification Notes

- `rg` scans confirmed there are no checked-in env examples, Docker-required env vars, compose
  files, Helm/Kubernetes manifests, Terraform modules, CI env declarations, README requirements, or
  package manifest dependencies to remove for deleted surfaces.
- `cargo metadata --locked --no-deps --format-version 1` passed, confirming manifest and lockfile
  consistency without dependency cleanup changes.
- Full build, tests, lint/typecheck, terminal smoke, and minimum-env startup checks should still be
  run as release gates. This stage did not change runtime code or dependency manifests.

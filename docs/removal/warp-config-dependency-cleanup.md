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

## Safe Removals Performed

- Removed dead referral GraphQL client modules that became unused after the app-side referral API
  client was deleted:
  - `crates/graphql/src/api/queries/get_referral_info.rs`
  - `crates/graphql/src/api/mutations/send_referral_invite_emails.rs`
- Removed their module exports from:
  - `crates/graphql/src/api/queries/mod.rs`
  - `crates/graphql/src/api/mutations/mod.rs`

## Remaining Live Env Var Usage

These env vars are still read by live code and were not removed in this stage:

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

Because those agent / harness / indexing code paths still exist in-tree, removing their env vars,
docs, or dependencies would be unsafe in this stage.

## Remaining Feature Flags Or Build Features Still In Tree

These are still present and tied to live code paths, so they were not removed here:

- `remote_codebase_indexing`
- `local_claude_codex_child_harnesses`
- `team_workflows`
- `team_api_keys`

## Verification Notes

- `cargo check -p warp --lib` is the verification command used for this stage.
- A full clean-install, full test suite, terminal smoke test, and minimum-env startup validation
  remain to be completed in a later pass once larger AI / agent / indexing removals are done.

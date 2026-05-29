# Warp DB Inventory

## Scope

This stage inventories the local SQLite persistence surface for the Warp-related product areas that
have been targeted for removal, without dropping tables or deleting data.

The repository's database layer is the embedded Diesel migration set under
`crates/persistence/migrations`, with the current live schema in:

- `crates/persistence/src/schema.rs`
- `crates/persistence/src/model.rs`
- `app/src/persistence/sqlite.rs`

## Key Findings

- The repo does not expose a separate server-side relational schema for these features. The active
  database surface is local SQLite persistence used by the desktop app.
- There is no dedicated persistence table for referrals, sign-in, sign-up, or product onboarding.
  Those surfaces are either not persisted in SQLite or are handled elsewhere.
- There is no archive convention in the persistence crate.
- There are no schedulers or background DB jobs in SQLite. The only feature-owned DB triggers I
  found are timestamp-maintenance triggers on `agent_conversations` and `agent_tasks`.
- A non-destructive cleanup migration is not needed in this stage. Adding one now would change
  persistence behavior for still-live code paths without reducing risk, and there is no existing
  pattern for "disable writes but keep schema" in this repo's SQLite layer.

## Current Target-Surface Inventory

### Feature-owned and safe to remove later

These objects are owned by the removed or removal-targeted product surfaces and are good later drop
targets once code references are gone.

| DB object | Surface | Why it is feature-owned | Primary references |
| --- | --- | --- | --- |
| `agent_conversations` | Agents / AI | Stores persisted agent conversation metadata only | `2025-06-09-013710_create_agent_conversations_table`, `app/src/persistence/agent.rs` |
| `agent_tasks` | Agents / AI | Stores persisted agent task protobufs only | `2025-06-09-013710_create_agent_conversations_table`, `app/src/persistence/agent.rs` |
| trigger `update_last_modified_at_for_agent_conversations` | Agents / AI | Only maintains agent-conversation timestamps | `2025-06-09-013710_create_agent_conversations_table/up.sql` |
| trigger `update_last_modified_at_for_agent_tasks` | Agents / AI | Only maintains agent-task timestamps | `2025-06-09-013710_create_agent_conversations_table/up.sql` |
| `ai_queries` | AI / ClaudePlatform | Persists AI query inputs and model choices | `2024-08-05-145004_add_ai_exchanges_table`, `2024-08-20-201322_split_ai_exchanges_table`, `app/src/persistence/block_list.rs` |
| `ai_memory_panes` | AI | Pane restoration for AI memory/facts UI only | `2025-01-28-175430_add_ai_memory_pane`, `app/src/persistence/sqlite.rs` |
| `ai_document_panes` | AI / Agents | Pane restoration for AI document UI only | `2025-10-22-020434_add_ai_document_pane`, `app/src/persistence/sqlite.rs` |
| `ambient_agent_panes` | Agents | Pane restoration for ambient/cloud agent UI only | `2026-02-19-154616-0000_add_ambient_agent_panes`, `app/src/persistence/sqlite.rs` |
| `team_settings` | Teams product settings | Local cache of organization/team settings | `2025-07-23-141051_create_team_settings_table`, `app/src/persistence/sqlite.rs` |
| `team_members` | Teams product metadata | Local cache of team-member lists | `2026-02-18-183600_add_members_json_to_teams`, `app/src/persistence/sqlite.rs` |

### Shared with terminal, auth, session, or workspace

These objects are used by core terminal execution, session restoration, auth, or workspace
selection. They should not be dropped in the feature-removal pass.

| DB object | Why it is shared | Primary references |
| --- | --- | --- |
| `app`, `windows`, `tabs`, `pane_nodes`, `pane_leaves`, `pane_branches`, `panels` | Core window/tab/pane restoration for terminal UI | base migrations plus `app/src/persistence/sqlite.rs` |
| `terminal_panes` | Terminal session restore data including shell launch, active profile, and conversation restore IDs | `2023-07-19-182041_generic_pane_leaves`, terminal-pane follow-up migrations, `app/src/persistence/block_list.rs` |
| `blocks` | Persisted terminal command/output blocks | `2021-12-15-212030_add_blocks`, `app/src/persistence/block_list.rs` |
| `commands` | Terminal command history and execution metadata | `2023-07-10-185435_add_commands_table`, `app/src/persistence/commands.rs` |
| `current_user_information` | Shared auth/session cache for current user identity | `2024-09-04-195712_add_current_user_information_table`, `app/src/persistence/sqlite.rs` |
| `workspaces` | Current workspace selection and local workspace cache used beyond removed team UI | `2024-12-30-232544_add_workspace_tables`, `2025-01-08-010739_add_current_workspace`, `app/src/persistence/sqlite.rs` |
| `workspace_teams` | Links current workspaces to cached teams; tied to workspace ownership and selection | `2024-12-30-232544_add_workspace_tables`, `app/src/persistence/sqlite.rs` |
| `workspace_metadata` | Originally codebase-index metadata, now shared with workspace/LSP state | `2025-10-31-201353_add_workspace_language_server`, `app/src/persistence/sqlite.rs` |
| `workspace_language_server` | Shared workspace editor/LSP enablement state | `2025-10-31-201353_add_workspace_language_server`, `app/src/persistence/sqlite.rs` |
| `projects`, `project_rules` | Project/workspace metadata used in active editor/terminal-adjacent flows | `2025-07-29-122627_add_projects_table`, `2025-08-13-181026_add_project_rules` |
| `teams` | Team metadata still participates in workspace ownership and billing/workspace gating | `2022-07-29-164904_add_team_workflows` onward, `app/src/persistence/sqlite.rs` |
| `user_profiles` | Shared identity metadata cached for workspace and collaboration features | `2023-08-07-205653_add_user_profiles_table` |
| `welcome_panes` | Startup/welcome pane restoration, not one of the removed surfaces | `2025-08-05-134035_add_data_to_welcome_panes` |

### Feature-owned columns embedded in shared tables

These columns look feature-owned, but they are embedded in otherwise shared terminal, window,
workspace, or command tables. They should only be dropped after the corresponding runtime readers
and writers are removed and covered by terminal restoration tests.

| DB object | Surface | Classification | Primary references |
| --- | --- | --- | --- |
| `blocks.ai_metadata` | AI / Agents | Risky embedded column on terminal command blocks; shared-session viewer still reconstructs command metadata from it | `2024-09-04-220259_add_blocks_ai_metadata`, `app/src/terminal/shared_session/viewer/event_loop.rs` |
| `blocks.agent_view_visibility` | Agents | Risky embedded column on terminal command blocks; active agent-view restore code still reads/writes it | `2026-01-15-163534-0000_add_agent_view_visibility_to_blocks`, `app/src/persistence/block_list.rs` |
| `commands.is_agent_executed` | Agents | Risky embedded column on terminal command history; keep until command-history readers no longer expose agent execution state | `2026-03-13-000000_add_is_agent_executed_to_commands`, `app/src/persistence/commands.rs` |
| `terminal_panes.conversation_ids` | AI / Agents | Risky embedded column on terminal pane restoration; active terminal pane snapshots still carry conversation restore IDs | `2025-11-05-220642_add_convo_id_terminal_panes`, `app/src/pane_group/pane/terminal_pane.rs` |
| `terminal_panes.active_conversation_id` | AI / Agents | Risky embedded column on terminal pane restoration; active terminal view code still tracks the selected conversation | `2026-02-09-024900_add_active_conversation_id_to_terminal_panes`, `app/src/pane_group/mod.rs` |
| `terminal_panes.llm_model_override` | AI / Agents | Risky embedded column on terminal pane restoration; model override state is still in terminal pane schema | `2025-08-20-113907_add_llm_model_override_to_terminal_panes`, `crates/persistence/src/schema.rs` |
| `windows.warp_ai_width` | AI | Risky embedded window-layout column; window restoration table is terminal-critical | `2023-03-21-191945_add_persisted_widths_for_warp_drive_etc`, `crates/persistence/src/schema.rs` |
| `windows.voltron_width` | AI / Agents | Risky embedded window-layout column; window restoration table is terminal-critical | `2023-03-21-191945_add_persisted_widths_for_warp_drive_etc`, `crates/persistence/src/schema.rs` |
| `windows.warp_drive_index_width` | WarpDrive | Risky embedded window-layout column; window restoration table is terminal-critical | `2023-03-21-191945_add_persisted_widths_for_warp_drive_etc`, `crates/persistence/src/schema.rs` |
| `windows.agent_management_filters` | Agents | Risky embedded window-layout column; active workspace view still initializes agent-management UI from it | `2026-01-06-113440-0000_add_agent_management_filters_to_windows`, `app/src/workspace/view.rs` |
| `notebooks.ai_document_id` | AI / Agents | Unknown/risky because notebooks are not fully feature-owned and still share cloud-object flows | `2025-10-31-115050_add_ai_document_id_to_notebooks`, `app/src/ai/document/ai_document_model.rs` |
| `teams.billing_metadata_json` | Teams / billing | Shared/risky because `teams` participates in workspace ownership and shared-session policy checks | `2026-02-18-021000_add_billing_metadata_to_teams`, `app/src/workspaces/user_workspaces.rs` |

### Unknown or risky

These objects have meaningful coupling outside the explicit removal list, or their ownership is
mixed enough that a later drop needs more isolation work first.

| DB object | Risk | Primary references |
| --- | --- | --- |
| `notebooks` | Historically tied to team/cloud sharing, but now also linked to AI documents | `2022-09-26-205723_create_notebooks_table`, `2025-10-31-115050_add_ai_document_id_to_notebooks` |
| `workflows` | Began as team workflow persistence, but still participates in command/workspace flows | `2022-07-29-164904_add_team_workflows`, `app/src/persistence/sqlite.rs` |
| `workflow_panes` | UI object for workflows, but backed by still-live shared object types | `2024-05-27-223416_add_workflow_pane_tables` |
| `object_metadata`, `object_permissions`, `object_actions`, `folders`, `generic_string_objects`, `cloud_objects_refreshes` | Shared cloud-object substrate spans drive, notebooks, workflows, and other surfaces not fully removed | multiple 2023 cloud-object migrations, `app/src/persistence/sqlite.rs` |
| `settings_panes` | Generic settings-pane restoration; not feature-owned even if some removed pages used it | `2025-01-15-174448_add-settings-panes` |
| `code_panes`, `code_pane_tabs`, `code_review_panes` | Editor/review surfaces are not in the requested drop list and may remain terminal-adjacent | `2024-05-21-183957_add-code-pane`, `2026-04-14-150000_add_code_pane_tabs`, `2025-09-29-154015_add_code_review_pane` |
| `mcp_server_installations`, `mcp_environment_variables`, `active_mcp_servers`, `mcp_server_panes` | AI-adjacent, but still live and not isolated from active tooling flows | 2025 MCP migrations, `app/src/persistence/sqlite.rs` |
| `env_var_collection_panes` | Product-adjacent rather than terminal-core, but not part of the requested removals | `2024-05-07-204616_add-env-var-panes` |
| `server_experiments`, `ignored_suggestions` | Cross-cutting state, not owned by the deleted surfaces alone | `2024-01-31-164120_add-server-experiments`, `2025-08-22-141948_create_ignored_suggestions_table` |

## Surfaces Not Present In SQLite

I did not find active SQLite tables or migrations specific to:

- Referrals / Refferals
- SignIn product onboarding
- SignUp product onboarding

The only onboarding-adjacent persistence I found is `welcome_panes`, which is a generic startup UI
restoration table and not a sign-in/sign-up account-onboarding table.

## Historical Objects Already Removed

These are relevant to the target surfaces but are already absent from the current schema:

- `ai_blocks`
  - introduced during AI persistence split
  - dropped by `2025-11-19-224140_remove_persisted_ai_blocks`
- `orchestration_lifecycle_events`
- `orchestration_messages`
- `orchestration_events`
  - dropped by `2026-03-23-180000_remove_orchestration_persistence`
- `codebase_index_metadata`
  - renamed/reworked into `workspace_metadata` by
    `2025-10-31-201353_add_workspace_language_server`

## Migration Decision For This Stage

No new migration was added.

Reasoning:

- There is no reversible "disable writes" convention in the persistence crate.
- The only feature-owned triggers are the agent timestamp triggers, and removing them while
  agent persistence tables remain live would silently change behavior rather than safely preparing
  cleanup.
- No DB scheduler/job framework exists here to disable.
- No archive-table convention exists to move data into.

The safe next step is to finish code-path removal first, then add a dedicated non-destructive
compatibility migration only if a later stage needs to preserve old SQLite files while making the
feature tables inert.

## Verification

- `diesel migration run --database-url /tmp/camael-stage6.sqlite --migration-dir crates/persistence/migrations --locked-schema`
  passed on a disposable SQLite database.
- `diesel migration redo --database-url /tmp/camael-stage6-latest-redo.sqlite --migration-dir crates/persistence/migrations --locked-schema`
  passed for the latest migration after applying all migrations to a disposable SQLite database.
- `diesel migration redo --all --database-url /tmp/camael-stage6.sqlite --migration-dir crates/persistence/migrations --locked-schema`
  failed on historical migration `2024-12-25-215548_drop_server_id_from_tables` because its down
  migration runs `ALTER TABLE teams ADD COLUMN server_id BIGINTEGER UNIQUE`, which SQLite rejects
  with `Cannot add a UNIQUE column`. This is pre-existing migration reversibility debt and was not
  introduced by this stage.

## Recommended Later Drop Order

1. Remove live code references to AI/agent pane restoration and agent conversation persistence.
2. Remove live code references to feature-owned columns embedded in shared terminal/window tables,
   then cover terminal session restoration and shared-session replay with tests.
3. Remove live code references to team settings and member caches if workspace ownership can be
   preserved without them.
4. Add a compatibility migration only if old app-state restoration would otherwise try to read the
   soon-to-be-removed tables.
5. Drop feature-owned tables, triggers, indexes, and proven-safe embedded columns in a later
   destructive schema stage.

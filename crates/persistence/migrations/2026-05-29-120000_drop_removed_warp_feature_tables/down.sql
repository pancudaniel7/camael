CREATE TABLE team_settings (
    id INTEGER PRIMARY KEY NOT NULL,
    team_id INTEGER NOT NULL UNIQUE,
    settings_json TEXT NOT NULL,
    FOREIGN KEY (team_id) REFERENCES teams (id)
);

CREATE TABLE team_members (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    team_id INTEGER NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    user_uid TEXT NOT NULL,
    email TEXT NOT NULL,
    role TEXT NOT NULL
);

CREATE TABLE ambient_agent_panes (
    id INTEGER PRIMARY KEY NOT NULL REFERENCES pane_nodes(id),
    kind TEXT NOT NULL DEFAULT 'ambient_agent',
    uuid BLOB NOT NULL,
    task_id TEXT
);

CREATE TABLE ai_document_panes (
    id INTEGER PRIMARY KEY NOT NULL,
    kind TEXT NOT NULL DEFAULT 'ai_document' CHECK (kind = 'ai_document'),
    document_id TEXT NOT NULL,
    version INTEGER NOT NULL,
    content TEXT,
    title TEXT,
    FOREIGN KEY (id, kind) REFERENCES pane_leaves (pane_node_id, kind)
);

CREATE TABLE ai_memory_panes (
    id INTEGER PRIMARY KEY NOT NULL,
    kind TEXT NOT NULL DEFAULT 'ai_memory' CHECK (kind = 'ai_memory'),
    FOREIGN KEY (id, kind) REFERENCES pane_leaves (pane_node_id, kind)
);

CREATE TABLE ai_queries (
    id INTEGER PRIMARY KEY NOT NULL,
    exchange_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL,
    start_ts TIMESTAMP NOT NULL,
    input TEXT NOT NULL,
    working_directory TEXT,
    output_status TEXT NOT NULL,
    model_id TEXT NOT NULL DEFAULT '',
    planning_model_id TEXT NOT NULL DEFAULT '',
    coding_model_id TEXT NOT NULL DEFAULT ''
);

CREATE TABLE agent_conversations (
    id INTEGER PRIMARY KEY NOT NULL,
    conversation_id TEXT NOT NULL,
    conversation_data TEXT NOT NULL,
    last_modified_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_last_modified_at_for_agent_conversations AFTER
UPDATE ON agent_conversations FOR EACH ROW WHEN NEW.last_modified_at IS OLD.last_modified_at BEGIN
UPDATE agent_conversations
SET
    last_modified_at = CURRENT_TIMESTAMP
WHERE
    id = OLD.id;

END;

CREATE UNIQUE INDEX ux_agent_conversations_conversation_id ON agent_conversations (conversation_id);

CREATE TABLE agent_tasks (
    id INTEGER PRIMARY KEY NOT NULL,
    conversation_id TEXT NOT NULL,
    task_id TEXT NOT NULL,
    task BLOB NOT NULL,
    last_modified_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (conversation_id) REFERENCES agent_conversations (conversation_id)
);

CREATE TRIGGER update_last_modified_at_for_agent_tasks AFTER
UPDATE ON agent_tasks FOR EACH ROW WHEN NEW.last_modified_at IS OLD.last_modified_at BEGIN
UPDATE agent_tasks
SET
    last_modified_at = CURRENT_TIMESTAMP
WHERE
    id = OLD.id;

END;

CREATE UNIQUE INDEX ux_agent_tasks_task_id ON agent_tasks (task_id);

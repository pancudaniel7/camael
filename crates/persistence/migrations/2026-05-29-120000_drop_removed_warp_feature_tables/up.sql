DROP TRIGGER IF EXISTS update_last_modified_at_for_agent_tasks;
DROP TRIGGER IF EXISTS update_last_modified_at_for_agent_conversations;

DROP TABLE IF EXISTS agent_tasks;
DROP TABLE IF EXISTS agent_conversations;
DROP TABLE IF EXISTS ai_queries;
DROP TABLE IF EXISTS ai_memory_panes;
DROP TABLE IF EXISTS ai_document_panes;
DROP TABLE IF EXISTS ambient_agent_panes;
DROP TABLE IF EXISTS team_members;
DROP TABLE IF EXISTS team_settings;

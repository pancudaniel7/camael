use diesel::SqliteConnection;
use warp_multi_agent_api as api;

use super::model::{AgentConversation, AgentConversationData};

#[derive(Debug, thiserror::Error)]
pub(super) enum UpsertConversationError {
    #[error("Failed to serialize conversation data: {0:?}")]
    Serialization(#[from] serde_json::Error),
    #[error("Failed to upsert conversation to sqlite: {0:?}")]
    DB(#[from] diesel::result::Error),
}

pub(super) fn upsert_agent_conversation<'a>(
    _conn: &mut SqliteConnection,
    _conversation_id_param: &str,
    _tasks: impl IntoIterator<Item = &'a api::Task>,
    _conversation_data_param: AgentConversationData,
) -> Result<(), UpsertConversationError> {
    // Agent persistence is intentionally disabled for the terminal-only product surface.
    Ok(())
}

pub(super) fn read_agent_conversations(
    _conn: &mut SqliteConnection,
) -> Result<Vec<AgentConversation>, diesel::result::Error> {
    Ok(Vec::new())
}

/// Read a single agent conversation by its ID, including decoded tasks.
pub(crate) fn read_agent_conversation_by_id(
    _conn: &mut SqliteConnection,
    _conversation_id_str: &str,
) -> Result<Option<AgentConversation>, diesel::result::Error> {
    Ok(None)
}

pub(super) fn delete_agent_conversations(
    _conn: &mut SqliteConnection,
    _conversation_ids: Vec<String>,
) -> Result<(), diesel::result::Error> {
    Ok(())
}

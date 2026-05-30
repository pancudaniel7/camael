use diesel::SqliteConnection;

use super::model::AgentConversation;

/// Read a single agent conversation by its ID, including decoded tasks.
pub(crate) fn read_agent_conversation_by_id(
    _conn: &mut SqliteConnection,
    _conversation_id_str: &str,
) -> Result<Option<AgentConversation>, diesel::result::Error> {
    Ok(None)
}

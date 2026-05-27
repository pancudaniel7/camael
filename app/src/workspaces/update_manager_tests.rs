use chrono::Utc;
use warpui::{AddSingletonModel, App};

use super::*;
use crate::auth::AuthManager;
use crate::cloud_object::model::actions::ObjectActions;
use crate::cloud_object::model::persistence::CloudModel;
use crate::cloud_object::{Revision, ServerMetadata, ServerPermissions, ServerWorkflow};
use crate::server::ids::SyncId;
use crate::server::server_api::workspace::WorkspaceClient;
use crate::server::sync_queue::SyncQueue;
use crate::server::telemetry::context_provider::AppTelemetryContextProvider;
use crate::settings::PrivacySettings;
use crate::system::SystemStats;
use crate::workflows::workflow::Workflow;
use crate::workflows::{CloudWorkflowModel, WorkflowId};
use crate::workspaces::user_profiles::UserProfiles;
use crate::workspaces::workspace::Workspace;

fn initialize_app(
    team_client: Arc<dyn TeamClient>,
    workspace_client: Arc<dyn WorkspaceClient>,
    workspaces: Vec<Workspace>,
    app: &mut App,
) {
    app.add_singleton_model(|_| NetworkStatus::new());
    app.add_singleton_model(|_| SystemStats::new());
    app.add_singleton_model(TeamTesterStatus::new);
    app.add_singleton_model(|ctx| {
        UserWorkspaces::mock(
            team_client.clone(),
            workspace_client.clone(),
            workspaces,
            ctx,
        )
    });
    app.add_singleton_model(SyncQueue::mock);
    app.add_singleton_model(CloudModel::mock);
    app.add_singleton_model(|_| ObjectActions::new(vec![]));
    app.add_singleton_model(PrivacySettings::mock);
    app.add_singleton_model(|_| UserProfiles::new(vec![]));
    app.add_singleton_model(|_| ServerApiProvider::new_for_test());
    app.add_singleton_model(|_| AuthStateProvider::new_for_test());
    app.add_singleton_model(AppTelemetryContextProvider::new_context_provider);
    app.add_singleton_model(AuthManager::new_for_test);
}

fn mock_server_workflow(id: WorkflowId) -> ServerWorkflow {
    ServerWorkflow::new(
        SyncId::ServerId(id.into()),
        CloudWorkflowModel::new(Workflow::new("Test Workflow", "echo hello")),
        ServerMetadata {
            uid: id.into(),
            revision: Revision::now(),
            metadata_last_updated_ts: Utc::now().into(),
            trashed_ts: None,
            folder_id: None,
            is_welcome_object: false,
            creator_uid: None,
            last_editor_uid: None,
            current_editor_uid: None,
        },
        ServerPermissions {
            space: crate::cloud_object::Owner::mock_current_user(),
            permissions_last_updated_ts: Utc::now().into(),
            anyone_link_sharing: None,
            guests: vec![],
        },
    )
}

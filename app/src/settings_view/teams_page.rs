use crate::server::ids::ServerId;
use crate::settings_view::settings_page::{MatchData, PageType, SettingsPageMeta, SettingsPageViewHandle};
use crate::view_components::ToastFlavor;
use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, TypedActionView, View, ViewContext, ViewHandle};

use super::SettingsSection;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TeamsInviteOption {
    #[default]
    Link,
    Email,
}

#[derive(Clone, Debug, Default)]
pub struct OpenTeamsSettingsModalArgs {
    pub invite_email: Option<String>,
}

#[derive(Debug, Clone)]
pub enum TeamsPageAction {
    NoOp,
    OpenWarpDrive,
    ChangeInviteViewOption(TeamsInviteOption),
    OpenAdminPanel { team_uid: ServerId },
}

pub enum TeamsPageViewEvent {
    TeamsChanged,
    OpenWarpDrive,
    ShowToast {
        message: String,
        flavor: ToastFlavor,
    },
}

pub struct TeamsPageView {
    page: PageType<Self>,
}

impl TeamsPageView {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self {
            page: PageType::new_uncategorized(vec![], None),
        }
    }

    pub fn open_team_members(&mut self, _email: Option<&String>, _ctx: &mut ViewContext<Self>) {}
}

impl Entity for TeamsPageView {
    type Event = TeamsPageViewEvent;
}

impl TypedActionView for TeamsPageView {
    type Action = TeamsPageAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

impl View for TeamsPageView {
    fn ui_name() -> &'static str {
        "RemovedTeamsSettingsPage"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        let _ = &self.page;
        Empty::new().finish()
    }
}

impl SettingsPageMeta for TeamsPageView {
    fn section() -> SettingsSection {
        SettingsSection::Teams
    }

    fn should_render(&self, _ctx: &AppContext) -> bool {
        false
    }

    fn update_filter(&mut self, _query: &str, _ctx: &mut ViewContext<Self>) -> MatchData {
        MatchData::Uncounted(false)
    }

    fn scroll_to_widget(&mut self, _widget_id: &'static str) {}

    fn clear_highlighted_widget(&mut self) {}
}

impl From<ViewHandle<TeamsPageView>> for SettingsPageViewHandle {
    fn from(view_handle: ViewHandle<TeamsPageView>) -> Self {
        SettingsPageViewHandle::Teams(view_handle)
    }
}

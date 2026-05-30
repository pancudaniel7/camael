use warpui::elements::{Element, Empty};
use warpui::keymap::ContextPredicate;
use warpui::{Action, AppContext, Entity, TypedActionView, View, ViewContext, ViewHandle};

use super::settings_page::{MatchData, PageType, SettingsPageMeta, SettingsPageViewHandle};
use super::SettingsSection;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AISubpage {
    WarpAgent,
    Profiles,
    Knowledge,
    ThirdPartyCLIAgents,
}

impl AISubpage {
    pub fn from_section(section: SettingsSection) -> Option<Self> {
        match section {
            SettingsSection::WarpAgent => Some(Self::WarpAgent),
            SettingsSection::AgentProfiles => Some(Self::Profiles),
            SettingsSection::Knowledge => Some(Self::Knowledge),
            SettingsSection::ThirdPartyCLIAgents => Some(Self::ThirdPartyCLIAgents),
            _ => None,
        }
    }
}

pub const fn cli_agent_settings_widget_id() -> &'static str {
    "removed_ai_settings_page"
}

#[derive(Clone, Debug)]
pub enum AISettingsPageAction {
    NoOp,
}

pub struct AISettingsPageView {
    page: PageType<Self>,
    active_subpage: Option<AISubpage>,
}

impl AISettingsPageView {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self {
            page: PageType::new_uncategorized(vec![], None),
            active_subpage: None,
        }
    }

    pub fn set_active_subpage(&mut self, subpage: Option<AISubpage>, ctx: &mut ViewContext<Self>) {
        self.active_subpage = subpage;
        ctx.notify();
    }
}

impl Entity for AISettingsPageView {
    type Event = ();
}

impl TypedActionView for AISettingsPageView {
    type Action = AISettingsPageAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

impl View for AISettingsPageView {
    fn ui_name() -> &'static str {
        "RemovedAISettingsPage"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        let _ = &self.page;
        let _ = self.active_subpage;
        Empty::new().finish()
    }
}

impl SettingsPageMeta for AISettingsPageView {
    fn section() -> SettingsSection {
        SettingsSection::AI
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

impl From<ViewHandle<AISettingsPageView>> for SettingsPageViewHandle {
    fn from(view_handle: ViewHandle<AISettingsPageView>) -> Self {
        SettingsPageViewHandle::AI(view_handle)
    }
}

pub fn init_actions_from_parent_view<T: Action + Clone>(
    _app: &mut AppContext,
    _context: &ContextPredicate,
    _builder: fn(super::SettingsAction) -> T,
) {
}

impl AISettingsPageView {
    pub fn get_modal_content(&self, _app: &AppContext) -> Option<Box<dyn Element>> {
        None
    }
}

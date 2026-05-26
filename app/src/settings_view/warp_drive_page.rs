use crate::settings_view::settings_page::{MatchData, PageType, SettingsPageMeta, SettingsPageViewHandle};
use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, TypedActionView, View, ViewContext, ViewHandle};

use super::SettingsSection;

#[derive(Debug, Clone)]
pub enum WarpDriveSettingsPageAction {
    NoOp,
    SignUp,
    OpenUrl(String),
    ToggleShowWarpDrive,
}

pub enum WarpDriveSettingsPageEvent {
    SignUp,
}

pub struct WarpDriveSettingsPageView {
    page: PageType<Self>,
}

impl WarpDriveSettingsPageView {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self {
            page: PageType::new_uncategorized(vec![], None),
        }
    }
}

impl Entity for WarpDriveSettingsPageView {
    type Event = WarpDriveSettingsPageEvent;
}

impl TypedActionView for WarpDriveSettingsPageView {
    type Action = WarpDriveSettingsPageAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        if let WarpDriveSettingsPageAction::SignUp = action {
            ctx.emit(WarpDriveSettingsPageEvent::SignUp);
        }
    }
}

impl View for WarpDriveSettingsPageView {
    fn ui_name() -> &'static str {
        "RemovedWarpDriveSettingsPage"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        let _ = &self.page;
        Empty::new().finish()
    }
}

impl SettingsPageMeta for WarpDriveSettingsPageView {
    fn section() -> SettingsSection {
        SettingsSection::WarpDrive
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

impl From<ViewHandle<WarpDriveSettingsPageView>> for SettingsPageViewHandle {
    fn from(view_handle: ViewHandle<WarpDriveSettingsPageView>) -> Self {
        SettingsPageViewHandle::WarpDrive(view_handle)
    }
}

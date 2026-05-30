use crate::settings_view::settings_page::{
    MatchData, PageType, SettingsPageMeta, SettingsPageViewHandle,
};
use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, TypedActionView, View, ViewContext, ViewHandle};

use super::SettingsSection;

#[derive(Clone, Debug)]
pub enum PlatformPageAction {}

pub struct PlatformPageView {
    page: PageType<Self>,
}

impl PlatformPageView {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self {
            page: PageType::new_uncategorized(vec![], None),
        }
    }
}

impl Entity for PlatformPageView {
    type Event = ();
}

impl TypedActionView for PlatformPageView {
    type Action = PlatformPageAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

impl View for PlatformPageView {
    fn ui_name() -> &'static str {
        "RemovedPlatformSettingsPage"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        let _ = &self.page;
        Empty::new().finish()
    }
}

impl SettingsPageMeta for PlatformPageView {
    fn section() -> SettingsSection {
        SettingsSection::OzCloudAPIKeys
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

impl From<ViewHandle<PlatformPageView>> for SettingsPageViewHandle {
    fn from(view_handle: ViewHandle<PlatformPageView>) -> Self {
        SettingsPageViewHandle::OzCloudAPIKeys(view_handle)
    }
}

impl PlatformPageView {
    pub fn get_modal_content(&self) -> Option<Box<dyn Element>> {
        None
    }
}

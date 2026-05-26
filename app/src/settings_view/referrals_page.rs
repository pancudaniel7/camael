use std::sync::Arc;

use crate::server::server_api::referral::ReferralsClient;
use crate::settings_view::settings_page::{MatchData, PageType, SettingsPageMeta, SettingsPageViewHandle};
use crate::view_components::ToastFlavor;
use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, TypedActionView, View, ViewContext, ViewHandle};

use super::SettingsSection;

#[derive(Debug)]
pub enum ReferralsPageAction {
    NoOp,
    SignupAnonymousUser,
}

pub enum ReferralsPageEvent {
    SignupAnonymousUser,
    FocusModal,
    ShowToast {
        message: String,
        flavor: ToastFlavor,
    },
}

pub struct ReferralsPageView {
    page: PageType<Self>,
}

impl ReferralsPageView {
    pub fn new(_referrals_client: Arc<dyn ReferralsClient>, _ctx: &mut ViewContext<Self>) -> Self {
        Self {
            page: PageType::new_uncategorized(vec![], None),
        }
    }
}

impl Entity for ReferralsPageView {
    type Event = ReferralsPageEvent;
}

impl TypedActionView for ReferralsPageView {
    type Action = ReferralsPageAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        if let ReferralsPageAction::SignupAnonymousUser = action {
            ctx.emit(ReferralsPageEvent::SignupAnonymousUser);
        }
    }
}

impl View for ReferralsPageView {
    fn ui_name() -> &'static str {
        "RemovedReferralsSettingsPage"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        let _ = &self.page;
        Empty::new().finish()
    }
}

impl SettingsPageMeta for ReferralsPageView {
    fn section() -> SettingsSection {
        SettingsSection::Referrals
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

impl From<ViewHandle<ReferralsPageView>> for SettingsPageViewHandle {
    fn from(view_handle: ViewHandle<ReferralsPageView>) -> Self {
        SettingsPageViewHandle::Referrals(view_handle)
    }
}

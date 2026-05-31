use super::SettingsSection;

/// A sidebar navigation item: a direct page link.
pub enum SettingsNavItem {
    /// A top-level page that is rendered directly in the sidebar.
    Page(SettingsSection),
}

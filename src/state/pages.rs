// pub mod home;
pub mod settings;
// use home::Home;
use settings::Settings;

use crate::message::pages::{PageTo, SettingsTo};
pub enum Page {
    Home,
    Settings(Settings),
}
impl From<PageTo> for Page {
    fn from(page_to: PageTo) -> Self {
        match page_to {
            PageTo::Home => Page::Home,
            PageTo::Settings(settings_to) => Page::Settings(match settings_to {
                SettingsTo::Common => Settings::Common,
                SettingsTo::Own => Settings::Own,
                SettingsTo::Audio => Settings::Audio,
                SettingsTo::About => Settings::About,
                SettingsTo::Advanced => Settings::Advanced,
            }),
        }
    }
}

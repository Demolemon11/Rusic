#[derive(Debug, Clone)]
pub enum PageTo {
    Home,
    Settings(SettingsTo),
}
#[derive(Debug, Clone)]

pub enum SettingsTo {
    Own,
    Audio,
    About,
    Common,
    Advanced,
}

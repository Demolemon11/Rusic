pub mod audio;
pub mod pages;
pub mod style;
use audio::Audio;
use pages::Page;
use style::Style;
pub struct State {
    pub audio_state: Audio,
    pub page_state: Page,
    pub style_state: Style,
    pub theme_state: iced::Theme,
}
impl Default for State {
    fn default() -> Self {
        let theme_state = iced::Theme::Dark;
        let audio_state = Audio::Pausing;
        let page_state = Page::Home;
        let style_state = Style::Standard;

        Self {
            theme_state,
            style_state,
            audio_state,
            page_state,
        }
    }
}

pub mod audio;
pub mod pages;
pub mod style;
pub mod theme;

#[derive(Debug, Clone)]
pub enum Message {
    Page(pages::PageTo),
    Theme(theme::ThemeTo),
    Style(style::StyleTo),
    Audio(audio::AudioTo),
}

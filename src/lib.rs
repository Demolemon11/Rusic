use iced::alignment::Horizontal;
use iced::theme;
use iced::widget::Text;
use iced::Length;
use iced::{
    widget::{column, container, Button},
    Sandbox, Theme,
};
use message::pages::SettingsTo;
use message::{audio::AudioTo, pages::PageTo, style::StyleTo, theme::ThemeTo, Message};
use state::pages::settings::Settings;
use state::pages::Page;
// use state::style::Style;
use state::State;
pub mod message;
pub mod state;

pub struct Application {
    state: state::State,
}

impl Sandbox for Application {
    type Message = message::Message;
    fn new() -> Self {
        let state = State::default();

        Self { state }
    }

    fn theme(&self) -> iced::Theme {
        self.state.theme_state.clone()
    }
    fn title(&self) -> String {
        String::from("Rusic")
    }
    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Audio(audio_to) => self.state.audio_state = audio_to.into(),
            Message::Page(page_to) => self.state.page_state = page_to.into(),
            Message::Theme(theme_to) => match theme_to {
                ThemeTo::Dark => {
                    if self.state.theme_state != Theme::Dark {
                        self.state.theme_state = Theme::Dark
                    }
                }
                ThemeTo::Light => {
                    if self.state.theme_state != Theme::Light {
                        self.state.theme_state = Theme::Light
                    }
                }
                ThemeTo::Moonfly => {
                    if self.state.theme_state != Theme::Moonfly {
                        self.state.theme_state = Theme::Moonfly
                    }
                }
                ThemeTo::Oxocarbon => {
                    if self.state.theme_state != Theme::Oxocarbon {
                        self.state.theme_state = Theme::Oxocarbon
                    }
                }
            },
            Message::Style(style_to) => self.state.style_state = style_to.into(),
        }
    }
    fn view(&self) -> iced::Element<'_, Self::Message> {
        match &self.state.page_state {
            Page::Home => container(column!(
                Button::new(
                    Text::from("ToSet")
                        .size(19)
                        .horizontal_alignment(Horizontal::Center)
                )
                .padding(40)
                .on_press(Message::Page(PageTo::Settings(SettingsTo::Own)))
                .style(theme::Button::custom(self.state.style_state.clone())),
                Button::new(
                    Text::from("Play")
                        .size(19)
                        .horizontal_alignment(Horizontal::Center)
                )
                .on_press(Message::Audio(AudioTo::Play))
                .style(theme::Button::custom(self.state.style_state.clone())),
                Button::new(
                    Text::from("Pause")
                        .size(19)
                        .horizontal_alignment(Horizontal::Center)
                )
                .on_press(Message::Audio(AudioTo::Pause))
                .style(theme::Button::custom(self.state.style_state.clone())),
                Button::new(
                    Text::from("Stop")
                        .size(19)
                        .horizontal_alignment(Horizontal::Center)
                )
                .on_press(Message::Audio(AudioTo::Stop))
                .style(theme::Button::custom(self.state.style_state.clone())),
            ))
            .align_x(Horizontal::Center)
            .height(Length::Fill)
            .width(Length::Fill)
            .into(),

            Page::Settings(settings) => match settings {
                Settings::Own => container(column!(
                    home_button().style(theme::Button::custom(self.state.style_state.clone())),
                    Button::new(
                        Text::from("Common")
                            .size(19)
                            .horizontal_alignment(Horizontal::Center)
                    )
                    .on_press(Message::Page(PageTo::Settings(SettingsTo::Common)))
                    .style(theme::Button::custom(self.state.style_state.clone())),
                    Button::new(
                        Text::from("button to std")
                            .size(19)
                            .horizontal_alignment(Horizontal::Center)
                    )
                    .on_press(Message::Style(StyleTo::Standard))
                    .style(theme::Button::custom(self.state.style_state.clone())),
                    Button::new(
                        Text::from("button to flashy")
                            .size(19)
                            .horizontal_alignment(Horizontal::Center)
                    )
                    .on_press(Message::Style(StyleTo::Flashy))
                    .style(theme::Button::custom(self.state.style_state.clone())),
                    Button::new(
                        Text::from("button to Lovely")
                            .size(19)
                            .horizontal_alignment(Horizontal::Center)
                    )
                    .on_press(Message::Style(StyleTo::Lovely))
                    .style(theme::Button::custom(self.state.style_state.clone())),
                    Button::new(
                        Text::from("Stop")
                            .size(19)
                            .horizontal_alignment(Horizontal::Center)
                    )
                    .on_press(Message::Audio(AudioTo::Stop))
                    .style(theme::Button::custom(self.state.style_state.clone())),
                ))
                .align_x(Horizontal::Center)
                .height(Length::Fill)
                .width(Length::Fill)
                .into(),
                Settings::Audio => container(column!(
                    home_button().style(theme::Button::custom(self.state.style_state.clone())),
                ))
                .align_x(Horizontal::Center)
                .height(Length::Fill)
                .width(Length::Fill)
                .into(),
                Settings::Common => container(column!(
                    home_button().style(theme::Button::custom(self.state.style_state.clone())),
                    Button::new(
                        Text::from("ThemeLight")
                            .size(19)
                            .horizontal_alignment(Horizontal::Left)
                    )
                    .on_press(Message::Theme(ThemeTo::Light))
                    .style(theme::Button::custom(self.state.style_state.clone())),
                    Button::new(
                        Text::from("ThemeDark")
                            .size(19)
                            .horizontal_alignment(Horizontal::Left)
                    )
                    .on_press(Message::Theme(ThemeTo::Dark))
                    .style(theme::Button::custom(self.state.style_state.clone())),
                    Button::new(
                        Text::from("ThemeMoonfly")
                            .size(19)
                            .horizontal_alignment(Horizontal::Left)
                    )
                    .on_press(Message::Theme(ThemeTo::Moonfly))
                    .style(theme::Button::custom(self.state.style_state.clone())),
                    Button::new(
                        Text::from("ThemeOxocarbon")
                            .size(19)
                            .horizontal_alignment(Horizontal::Left)
                    )
                    .on_press(Message::Theme(ThemeTo::Oxocarbon))
                    .style(theme::Button::custom(self.state.style_state.clone())),
                ))
                .align_x(Horizontal::Center)
                .height(Length::Fill)
                .width(Length::Fill)
                .into(),
                Settings::Advanced => container(Text::new("Developing").size(30))
                    .align_x(Horizontal::Center)
                    .height(Length::Fill)
                    .width(Length::Fill)
                    .into(),
                Settings::About => container(column!(
                    home_button().style(theme::Button::custom(self.state.style_state.clone())),
                    Text::new("ToBeDeveloping").size(30),
                ))
                .align_x(Horizontal::Center)
                .height(Length::Fill)
                .width(Length::Fill)
                .into(),
            },
        }
    }
}

fn home_button() -> Button<'static, Message> {
    Button::new(
        Text::from("ToHome")
            .size(19)
            .horizontal_alignment(Horizontal::Center),
    )
    .on_press(Message::Page(PageTo::Home))
}

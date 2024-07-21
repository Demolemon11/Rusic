use std::fs::{self, File};
// use std::io::BufReader;
// use std::path::Path;

use audio_folder::AudiosInfo;
use iced::alignment::Horizontal;
use iced::widget::{Container, Text};
use iced::{executor, Command, Length};
use iced::{theme, Application};
use iced::{
    widget::{column, container, Button},
    Theme,
};
use message::pages::SettingsTo;
use message::{audio::AudioTo, pages::PageTo, style::StyleTo, theme::ThemeTo, Message};
use rodio::{Decoder, OutputStream, Sink};

use state::pages::settings::Settings;
use state::pages::Page;
use state::State;
pub mod audio_folder;
mod message;
mod state;
pub struct App {
    state: state::State,
    sink: Sink,
}

impl Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let state = State::default();

        // TODO 循环添加要播放的音频路径

        let (stream, handle) = OutputStream::try_default().unwrap();
        Box::leak(Box::new(stream));

        let sink = Sink::try_new(&handle).unwrap();
        sink.pause();

        // let audio_path = Path::new("music/music_2.mp3");
        // let data = BufReader::new(File::open(audio_path).unwrap());
        // let source = Decoder::new(data).unwrap();
        // sink.append(source);

        (Self { state, sink }, Command::none())
    }

    fn theme(&self) -> iced::Theme {
        self.state.theme_state.clone()
    }

    fn title(&self) -> String {
        String::from("Rusic")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Audio(audio_to) => match audio_to {
                AudioTo::Play => {
                    self.sink.play();
                }
                AudioTo::Stop => self.sink.clear(),

                AudioTo::Pause => {
                    self.sink.pause();
                }
            },
            Message::Page(page_to) => self.state.page_state = page_to.into(),
            Message::Theme(theme_to) => match theme_to {
                ThemeTo::Dark => self.state.theme_state = Theme::Dark,
                ThemeTo::Light => self.state.theme_state = Theme::Light,
                ThemeTo::Moonfly => self.state.theme_state = Theme::Moonfly,
                ThemeTo::Oxocarbon => self.state.theme_state = Theme::Oxocarbon,
            },
            Message::Style(style_to) => self.state.style_state = style_to.into(),
            Message::OpenFolder => {
                let audios_json = serde_json::to_string(&AudiosInfo::default().0).unwrap();
                println!("Serilize Success");
                fs::write("playlist.json", audios_json).unwrap()
            }
        };
        Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        match &self.state.page_state {
            Page::Home => self.home_page().into(),
            Page::Settings(settings) => self.settings_page(settings).into(),
        }
    }

    fn style(&self) -> <Self::Theme as iced::application::StyleSheet>::Style {
        <Self::Theme as iced::application::StyleSheet>::Style::default()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::none()
    }
}
fn home_button() -> Button<'static, Message> {
    Button::new(
        Text::from("ToHome")
            .size(19)
            .horizontal_alignment(Horizontal::Left),
    )
    .on_press(Message::Page(PageTo::Home))
}

impl App {
    fn home_page(&self) -> Container<Message> {
        container(column!(
            Button::new(
                Text::from("Add Your Music Folder")
                    .size(19)
                    .horizontal_alignment(Horizontal::Left)
            )
            .padding(40)
            .on_press(Message::OpenFolder)
            .style(theme::Button::custom(self.state.style_state.clone())),
            Button::new(
                Text::from("ToSet")
                    .size(19)
                    .horizontal_alignment(Horizontal::Left)
            )
            .padding(30)
            .on_press(Message::Page(PageTo::Settings(SettingsTo::Own)))
            .style(theme::Button::custom(self.state.style_state.clone())),
            Button::new(
                Text::from("Play")
                    .size(19)
                    .horizontal_alignment(Horizontal::Left)
            )
            .on_press(Message::Audio(AudioTo::Play))
            .style(theme::Button::custom(self.state.style_state.clone())),
            Button::new(
                Text::from("Pause")
                    .size(19)
                    .horizontal_alignment(Horizontal::Left)
            )
            .on_press(Message::Audio(AudioTo::Pause))
            .style(theme::Button::custom(self.state.style_state.clone())),
            Button::new(
                Text::from("Stop")
                    .size(19)
                    .horizontal_alignment(Horizontal::Left)
            )
            .on_press(Message::Audio(AudioTo::Stop))
            .style(theme::Button::custom(self.state.style_state.clone())),
        ))
        .align_x(Horizontal::Left)
        .height(Length::Fill)
        .width(Length::Fill)
    }
    fn settings_page(&self, settings: &Settings) -> Container<Message> {
        match settings {
            Settings::Own => container(column!(
                home_button().style(theme::Button::custom(self.state.style_state.clone())),
                Button::new(
                    Text::from("Common")
                        .size(19)
                        .horizontal_alignment(Horizontal::Left)
                )
                .on_press(Message::Page(PageTo::Settings(SettingsTo::Common)))
                .style(theme::Button::custom(self.state.style_state.clone())),
                Button::new(
                    Text::from("button to std")
                        .size(19)
                        .horizontal_alignment(Horizontal::Left)
                )
                .on_press(Message::Style(StyleTo::Standard))
                .style(theme::Button::custom(self.state.style_state.clone())),
                Button::new(
                    Text::from("button to flashy")
                        .size(19)
                        .horizontal_alignment(Horizontal::Left)
                )
                .on_press(Message::Style(StyleTo::Flashy))
                .style(theme::Button::custom(self.state.style_state.clone())),
                Button::new(
                    Text::from("button to Lovely")
                        .size(19)
                        .horizontal_alignment(Horizontal::Left)
                )
                .on_press(Message::Style(StyleTo::Lovely))
                .style(theme::Button::custom(self.state.style_state.clone())),
                Button::new(
                    Text::from("Stop")
                        .size(19)
                        .horizontal_alignment(Horizontal::Left)
                )
                .on_press(Message::Audio(AudioTo::Stop))
                .style(theme::Button::custom(self.state.style_state.clone())),
            )),
            Settings::Audio => container(column!(
                home_button().style(theme::Button::custom(self.state.style_state.clone())),
            )),
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
            )),

            Settings::Advanced => container(Text::new("Developing").size(30)),
            Settings::About => container(column!(
                home_button().style(theme::Button::custom(self.state.style_state.clone())),
                Text::new("ToBeDeveloping").size(30),
            )),
        }
        .align_x(Horizontal::Left)
        .height(Length::Fill)
        .width(Length::Fill)
    }
}

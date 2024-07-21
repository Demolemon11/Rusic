use iced::{widget::button, Border, Shadow, Theme};

use crate::message::style::StyleTo;

#[derive(PartialEq, Eq, Clone)]
pub enum Style {
    Standard,
    Lovely,
    Flashy,
}
impl From<StyleTo> for Style {
    fn from(style_to: StyleTo) -> Self {
        match style_to {
            StyleTo::Standard => Style::Standard,
            StyleTo::Lovely => Style::Lovely,
            StyleTo::Flashy => Style::Flashy,
        }
    }
}
impl button::StyleSheet for Style {
    type Style = iced::Theme;
    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(match self {
                Self::Standard => iced::Color::from_rgb8(100, 100, 100),
                Self::Lovely => iced::Color::from_rgb8(255, 0, 0),
                Self::Flashy => iced::Color::from_rgb8(0, 0, 255),
            })),
            text_color: match style {
                Theme::Light | Theme::Oxocarbon => iced::Color::BLACK,
                Theme::Dark | Theme::Moonfly => iced::Color::WHITE,
                _ => iced::Color::default(),
            },
            border: match self {
                Self::Standard => Border::default(),
                Self::Lovely => Border::with_radius(20),
                Self::Flashy => Border::with_radius(-5),
            },
            shadow: match self {
                Self::Standard => Shadow::default(),
                Self::Lovely => Shadow {
                    color: iced::Color::from_rgba8(250, 250, 250, 0.5),
                    blur_radius: 0.7,
                    ..Default::default()
                },
                Self::Flashy => Shadow {
                    color: iced::Color::from_rgba8(100, 100, 100, 0.3),
                    blur_radius: 0.0,
                    ..Default::default()
                },
            },
            ..Default::default()
        }
    }
}

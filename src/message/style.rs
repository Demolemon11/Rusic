#[derive(Debug, Clone)]
pub enum StyleTo {
    Standard,
    Lovely,
    Flashy,
}
// impl button::StyleSheet for StyleTo {
//     type Style = iced::Theme;
//     fn active(&self, style: &Self::Style) -> button::Appearance {
//         button::Appearance {
//             background: Some(iced::Background::Color(match self {
//                 Self::Standard => iced::Color::from_rgb8(100, 100, 100),
//                 Self::Lovely => iced::Color::from_rgb8(255, 0, 0),
//                 Self::Flashy => iced::Color::from_rgb8(0, 0, 255),
//             })),
//             text_color: match style {
//                 Theme::Light | Theme::Oxocarbon => iced::Color::BLACK,
//                 Theme::Dark | Theme::Moonfly => iced::Color::WHITE,
//                 _ => iced::Color::default(),
//             },
//             border: match self {
//                 Self::Standard => Border::default(),
//                 Self::Lovely => Border::with_radius(20),
//                 Self::Flashy => Border::with_radius(-5),
//             },
//             shadow: match self {
//                 Self::Standard => Shadow::default(),
//                 Self::Lovely => Shadow {
//                     color: iced::Color::from_rgba8(250, 250, 250, 0.5),
//                     offset: Vector::new(0.2, 0.2),
//                     blur_radius: 0.7,
//                 },
//                 Self::Flashy => Shadow {
//                     color: iced::Color::from_rgba8(100, 100, 100, 0.3),
//                     offset: Vector::new(0.0, 0.0),
//                     blur_radius: 0.0,
//                 },
//             },
//             ..Default::default()
//         }
//     }
// }

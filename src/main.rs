use iced::{Sandbox, Settings};
use rusic::Application;
// #[derive(Debug, Clone)]
// enum Message {
//     Increase,
//     Decrease,
//     ToggleTheme,
//     GoToSettings,
//     GoToMain,
//     ToggleStyle(ToggleStyleTo),
// }
// #[derive(Debug, Clone, Debug, Clone)]
// enum CustomStyle {
//     Standard,
//     Lovely,
//     Flashy,
// }
fn main() {
    Application::run(Settings::default()).unwrap()
}

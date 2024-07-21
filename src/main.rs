// #![windows_subsystem = "windows"]

use iced::{Application, Settings};

use rusic::App;

fn main() {
    App::run(Settings::default())
        //TODO err handle
        .unwrap()
}

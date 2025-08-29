
use macroquad::prelude::Conf;
use crate::controller::Controller;
mod controller;
mod gui;
mod components;
mod assets;

/// The configuration of the application window
fn window_conf() -> Conf {
    Conf {
        fullscreen: true,
        window_resizable: false,
        ..Default::default()
    }
}


/// Starting point of the program
#[macroquad::main(window_conf)]
async fn main() {

    let mut controller = Controller::new();

    controller.start().await;

}








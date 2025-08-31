
use macroquad::prelude::Conf;
use kingdomino::controller::Controller;

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

    let mut controller = Controller::new().await;
    controller.start().await;

}








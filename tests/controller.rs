use kingdomino::controller::Controller;

#[macroquad::test]
/// Tests initializing the controller a bunch of times. Makes sure the fist draft has no null dominoes etc.
async fn initiate_controller() {


    for i in 0..100 {
        let controller = Controller::new().await;
    }

}
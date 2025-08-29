use macroquad::input::{is_key_pressed, is_mouse_button_pressed, MouseButton};
use macroquad::prelude::draw_texture_ex;
use crate::components::domino::Domino;
use crate::components::draft::Draft;








/// Holds the constants related to the display of the draft
mod draft {
    const DRAFT_POSITION: (f32, f32) = (10.0, 10.0);
    const DRAFT_DOMINO_SIZE: f32 = 40.0;

    // The vertical spacing between the draft dominoes
    const DRAFT_POSITION_OFFSET: f32 = 50.0;
}













pub(crate) struct Gui {




}






impl Gui {

    pub(crate) fn new() -> Self {
        Self {

        }
    }

    pub(crate) fn picked_domino() -> Option<Domino> {

        if is_mouse_button_pressed(MouseButton::Left) {
            // TODO: Handle input to check if we clicked on one of the dominoes in the draft


        }


        None
    }

    pub(crate) fn draw(&self, draft: &Draft) {

        self.draw_draft(draft)

    }

    fn draw_draft(&self, draft: &Draft) {


        for domino in draft.iter() {


            // TODO: fill with proper values
            Self::draw_domino(domino, 0.0, 0.0, 0.0)



        }

    }


    fn draw_domino(domino: &Domino, tile_size: f32, x: f32, y: f32) {

        // TODO: Get the texture of the domino from some asset dictionary
        // let texture =


        // draw_texture_ex(&*texture, x + self.x_offset, y + self.y_offset, WHITE, DrawTextureParams {
        //     dest_size: Some(Vec2::new(80.0, 80.0)), // target size
        //     ..Default::default()
        // }, );


    }






}
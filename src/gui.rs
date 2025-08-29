use std::any::Any;
use macroquad::color::WHITE;
use macroquad::input::{is_key_pressed, is_mouse_button_pressed, MouseButton};
use macroquad::prelude::{clear_background, draw_texture_ex, DrawTextureParams, Vec2};
use crate::assets::Assets;
use crate::components::domino::Domino;
use crate::components::draft::Draft;



mod board_gui {
    use macroquad::prelude::Color;

    pub(crate) const BACKGROUND_COLOR: Color = Color::from_rgba(36, 36, 36, 255);

}





/// Holds the constants related to the display of the draft
mod draft_gui {
    pub(crate) const TOP_DOMINO_POSITION: (f32, f32) = (150.0, 150.0);
    pub(crate) const DOMINO_SIZE: f32 = 100.0;

    // The vertical spacing between the draft dominoes
    pub(crate) const VERT_OFFSET: f32 = 120.0;
}




pub(crate) struct Gui {

    assets: Assets,


}






impl Gui {

    pub(crate) async fn new() -> Self {
        Self {
            assets: Assets::load().await,
        }
    }

    pub(crate) fn picked_domino() -> Option<Domino> {

        if is_mouse_button_pressed(MouseButton::Left) {
            // TODO: Handle input to check if we clicked on one of the dominoes in the draft


        }


        None
    }

    pub(crate) fn draw(&self, draft: &Draft) {
        clear_background(board_gui::BACKGROUND_COLOR);


        self.draw_draft(draft)

    }

    fn draw_draft(&self, draft: &Draft) {


        for (i, domino) in draft.iter().enumerate() {

            self.draw_domino(domino, draft_gui::DOMINO_SIZE,
                              draft_gui::TOP_DOMINO_POSITION.0,
                              draft_gui::TOP_DOMINO_POSITION.1 + (i as f32 * draft_gui::VERT_OFFSET))



        }

    }


    fn draw_domino(&self, domino: &Domino, tile_size: f32, x: f32, y: f32) {

        // TODO: Get the texture of the domino from some asset dictionary
        let texture = self.assets.fetch_domino_texture_by_id(domino.id());

        // Make sure we actually got the texture back!
        debug_assert_ne!(texture, None);

        let texture = texture.unwrap(); // extract from Some(texture) -> texture


        // Draw the texture on the board. x is twice as large as y, by nature of what a domino is and our orientation.
        draw_texture_ex(texture, x, y, WHITE, DrawTextureParams {
            dest_size: Some(Vec2::new(tile_size*2.0, tile_size)),
            ..Default::default()
        }, );
    }






}
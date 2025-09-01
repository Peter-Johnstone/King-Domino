use std::any::Any;
use macroquad::color::WHITE;
use macroquad::input::{is_key_pressed, is_mouse_button_pressed, MouseButton};
use macroquad::prelude::{clear_background, draw_texture_ex, mouse_position, screen_height, DrawTextureParams, Vec2};
use macroquad::window::screen_width;
use crate::assets::Assets;
use crate::components::domino::Domino;
use crate::components::draft;
use crate::components::draft::{Draft, DRAFT_SIZE};
use crate::components::player::Player;

mod board_gui {
    use macroquad::prelude::Color;

    pub(crate) const BACKGROUND_COLOR: Color = Color::from_rgba(36, 36, 36, 255);

}





/// Holds the constants related to the display of the draft
mod draft_gui {

    pub(crate) const PICK_DOMINO_X: f32 = 100.0;
    pub(crate) const PLACE_DOMINO_X: f32 = 320.0;
    pub(crate) const DOMINO_TILE_SIZE: f32 = 100.0;

    // The vertical spacing between the draft dominoes
    pub(crate) const VERT_OFFSET: f32 = 117.0;
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


    /// Returns the domino if we clicked a domino in the draft
    pub(crate) fn picked_draft_domino(draft: &mut Draft, cur_player: &Player) -> Option<Domino> {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();

            // X bounds of the draft area
            let x_min = draft_gui::PICK_DOMINO_X;
            let x_max = draft_gui::PICK_DOMINO_X + 2.0 * draft_gui::DOMINO_TILE_SIZE;

            if mx >= x_min && mx <= x_max {
                let top = Self::top_draft_domino_y();

                for i in 0..DRAFT_SIZE {
                    let y_min = top + i as f32 * draft_gui::VERT_OFFSET;
                    let y_max = y_min + draft_gui::DOMINO_TILE_SIZE;

                    if my >= y_min && my <= y_max && draft.pickable(i) {
                        return Some(draft.pick(i, cur_player));
                    }
                }
            }
        }
        None
    }


    /// The overarching draw function. Called each frame of the game.
    pub(crate) fn draw(&self, pick_draft: &Draft, place_draft: &Draft) {
        clear_background(board_gui::BACKGROUND_COLOR);

        self.draw_draft(pick_draft, draft_gui::PICK_DOMINO_X);

        if !place_draft.is_null() {
            self.draw_draft(place_draft, draft_gui::PLACE_DOMINO_X);

        }
    }

    /// Returns the y coordinate of the top domino of the draft. Calculated based on screen height and draft size.
    fn top_draft_domino_y() -> f32 {
        // Half screen plus the two dominoes above the halfway point
        screen_height()/2.0 - (DRAFT_SIZE as f32/2.0) * draft_gui::VERT_OFFSET
    }

    fn draw_draft(&self, draft: &Draft, domino_x: f32) {

        if draft.is_null() {
            return;
        }

        let top_domino_y = Self::top_draft_domino_y();

        for (i, domino) in draft.iter().enumerate() {

            self.draw_domino(domino, draft_gui::DOMINO_TILE_SIZE,
                             domino_x,
                             top_domino_y + (i as f32 * draft_gui::VERT_OFFSET));

            if let Some(player) = draft.player_on(i) {
                self.draw_king_on_domino(player.my_turn().idx(),
                                         domino_x,
                                         top_domino_y + (i as f32 * draft_gui::VERT_OFFSET));
            }


        }

    }


    /// Takes the coordinates of a domino and draws the king meeple on top of it. The player idx determines color.
    fn draw_king_on_domino(&self, player_idx: usize, domino_x: f32, domino_y: f32) {

        let texture = self.assets.fetch_king_texture_by_turn(player_idx as u8);

        debug_assert_ne!(texture, None);

        let texture = texture.unwrap(); // extract from Some(texture) -> texture

        // Size of the king drawn is scaled off of the
        const SIZE: f32 = draft_gui::DOMINO_TILE_SIZE/2.5;

        // Draw the texture on the board. x is twice as large as y, by nature of what a domino is and our orientation.
        draw_texture_ex(texture, domino_x + draft_gui::DOMINO_TILE_SIZE - SIZE/2.0,
                        domino_y + draft_gui::DOMINO_TILE_SIZE/2.0 - SIZE/2.0, WHITE, DrawTextureParams {
            dest_size: Some(Vec2::new(SIZE, SIZE)),
            ..Default::default()
        }, );
    }

    fn draw_domino(&self, domino: &Domino, tile_size: f32, x: f32, y: f32) {

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
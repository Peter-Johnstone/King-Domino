use std::any::Any;
use macroquad::color::WHITE;
use macroquad::input::{is_mouse_button_pressed, MouseButton};
use macroquad::prelude::*;
use crate::assets::Assets;
use crate::controller::Phase;
use crate::components::domino::Domino;
use crate::components::draft::{Draft, DRAFT_SIZE};
use crate::components::player::Player;
use crate::gui::board_gui::SCROLL_SIZE;
use crate::gui::text_bank::{PICKING_ADVICE, PLACING_ADVICE};

mod board_gui {
    use macroquad::prelude::Color;

    pub(crate) const BACKGROUND_COLOR: Color = Color::from_rgba(36, 36, 36, 255);
    pub(crate) const ACCENT_COLOR: Color = Color::from_rgba(168,168,168, 255);
    pub(crate) const BLUE: Color = Color::from_rgba(51, 121, 197, 255);
    pub(crate) const GREEN: Color = Color::from_rgba(50, 176, 92, 255);
    pub(crate) const RED: Color = Color::from_rgba(238, 131, 138, 255);
    pub(crate) const YELLOW: Color = Color::from_rgba(232, 189, 2, 255);
    pub(crate) const SCROLL_SIZE: f32 = 75.0;
    pub(crate) const SCORE_KING_SIZE: f32 = 75.0;
}

mod text_bank {
    pub(crate) const PICKING_ADVICE: &str = "
    Phase: Picking\n
    Click on a domino to place a king there.\n
    Active Player:
    ";
    pub(crate) const PLACING_ADVICE: &str = "
    Phase: Placing\n
    press 'r' to rotate, click to place\n
    domino into a socket. Available sockets\n
    are only shown for the current orientation.\n
    A socket represents where the TILE WITH\n
    THE HAND ON IT will be placed\n
    Active Player:
    ";
}



/// Holds the constants related to the display of the draft
mod draft_gui {

    pub(crate) const PICK_DOMINO_X: f32 = 100.0;
    pub(crate) const PLACE_DOMINO_X: f32 = 220.0;
    pub(crate) const DOMINO_TILE_SIZE: f32 = 50.0;

    // The vertical spacing between the draft dominoes
    pub(crate) const VERT_OFFSET: f32 = 60.0;
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

    /// Creates the container lines
    pub(crate) fn make_containers(&self){
        let color = board_gui::ACCENT_COLOR;
        // Draw container lines
        draw_line(0.0, screen_height()/2.0-100.0, screen_width()/3.0, screen_height()/2.0-100.0, 10.0, color);  //hori
        draw_line(0.0, screen_height()/2.0+50.0, screen_width()/3.0, screen_height()/2.0+50.0, 10.0, color);  //hori

        draw_line(screen_width()/3.0, 0.0, screen_width()/3.0, screen_height(), 10.0, color);       //virt
        // Draw player-pane subdivision lines
        draw_line(screen_width()/3.0, screen_height()/2.0, screen_width(), screen_height()/2.0, 5.0, color);//hori
        draw_line(screen_width()*(2.0/3.0), 0.0, screen_width()*(2.0/3.0), screen_height(), 5.0, color);    //virt

        // Draw scrolls
        self.draw_obj(self.assets.fetch_draft_scroll(), 0.0,(screen_height()/4.0)-75.0, board_gui::SCROLL_SIZE);
        self.draw_obj(self.assets.fetch_score_scroll(), 0.0, (screen_height()*(3.0/4.0))-50.0, board_gui::SCROLL_SIZE);

        // Draw king icons on score box
        for idx in 1..5 {
            let i: f32 = idx as f32;
            self.draw_obj(self.assets.fetch_king_texture_by_turn(idx), 100.0, (screen_height()*(3.0/4.0)-200.0+i*75.0), board_gui::SCORE_KING_SIZE);
        }

        // Draw colored borders within grid panes
        self.draw_color_border(board_gui::BLUE,   -1.0, -1.0);
        self.draw_color_border(board_gui::GREEN,   1.0, -1.0);
        self.draw_color_border(board_gui::RED,    -1.0,  1.0);
        self.draw_color_border(board_gui::YELLOW,  1.0,  1.0);

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
                        return Some(draft.pick(i, cur_player.id()));
                    }
                }
            }
        }
        None
    }


    /// The overarching draw function. Called each frame of the game.
    pub(crate) fn draw(&self, pick_draft: &Draft, place_draft: &Draft, active_player_id: &usize, phase: &Phase) {
        clear_background(board_gui::BACKGROUND_COLOR);
        self.make_containers();
        self.add_advice_box(*active_player_id, phase); 
        self.draw_draft(pick_draft, draft_gui::PICK_DOMINO_X);

        if !place_draft.is_null() {
            self.draw_draft(place_draft, draft_gui::PLACE_DOMINO_X);

        }
    }

    /// Returns the y coordinate of the top domino of the draft. Calculated based on screen height and draft size.
    fn top_draft_domino_y() -> f32 {
        // Half screen plus the two dominoes above the halfway point
        screen_height()/4.0 - 50.0 - (DRAFT_SIZE as f32/2.0) * draft_gui::VERT_OFFSET
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

            if let Some(id) = draft.player_on(i) {
                self.draw_king_on_domino(id as usize,
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

    // Draws a scroll on the screen
    fn draw_obj(&self, texture: Option<&Texture2D>, x: f32, y: f32, size: f32){
        debug_assert_ne!(texture, None);
        let texture = texture.unwrap(); // extract from Some(texture) -> texture
        draw_texture_ex(texture, x, y, WHITE, DrawTextureParams {
            dest_size: Some(Vec2::new(size, size)),
            ..Default::default()
        }, );
    }

    // Draws a border around each pane
    fn draw_color_border(&self, color: Color, x: f32, y: f32){
        //math stuff
        let mut x_offset: f32 = screen_width()*(2.0/3.0);
        let mut y_offset: f32 = screen_height()/2.0;
        x_offset = x_offset + x*screen_width()/6.0;
        y_offset = y_offset + y*screen_height()/4.0;
        let x_lef_wall = x_offset - screen_width()/6.0 + 5.0;
        let x_rig_wall = x_offset + screen_width()/6.0 - 5.0;
        let y_top_wall = y_offset - screen_height()/4.0 + 5.0;
        let y_bot_wall = y_offset + screen_height()/4.0 - 5.0;

        //macroquad fn call
        draw_line(x_lef_wall, y_top_wall, x_rig_wall, y_top_wall, 5.0, color);//top
        draw_line(x_lef_wall, y_bot_wall, x_rig_wall, y_bot_wall, 5.0, color);//bottom
        draw_line(x_lef_wall, y_top_wall, x_lef_wall, y_bot_wall, 5.0, color);//left
        draw_line(x_rig_wall, y_top_wall, x_rig_wall, y_bot_wall, 5.0, color);//right
    }

    // Adds the advice text and active king sprite to the box on the left hand side and halfway down screen
    fn add_advice_box(&self, idx: usize, phase: &Phase){
        //Gets the right text based on game phase
        let mut curr_advice: String;
        match phase {
            Phase::Placing => {
                curr_advice = String::from(PLACING_ADVICE);
            }
            &Phase::Picking => {
                curr_advice = String::from(PICKING_ADVICE);
            }
        }
        //Draw text
        draw_multiline_text(&curr_advice, -10.0, screen_height()/2.0 - 75.0, 20.0, Some(0.3), WHITE);
        
        //Draw king of active player
        self.draw_obj(self.assets.fetch_king_texture_by_turn(idx as u8), screen_width()/3.0-50.0, screen_height()/2.0, 30.0);
    }

}
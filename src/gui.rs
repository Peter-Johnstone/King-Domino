use std::f64::consts::PI;
use macroquad::color::WHITE;
use macroquad::input::{is_mouse_button_pressed, MouseButton};
use macroquad::prelude::*;
use crate::assets::Assets;
use crate::components::{grid, grid_domino};
use crate::components::grid_domino::GridDomino;
use crate::controller::Phase;
use crate::components::domino::Domino;
use crate::components::draft::{Draft, DRAFT_SIZE};
use crate::components::player::Player;
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

mod grid_multipliers_gui {
    pub(crate) const X_MULTIPLIER: f32 = 50.0;
    pub(crate) const Y_MULTIPLIER: f32 = 50.0;
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

enum PlacementDominoRotation {
    UP,
    LEFT,
    DOWN,
    RIGHT
}
impl PlacementDominoRotation{
    fn next(&self) -> Self {
        match *self {
            PlacementDominoRotation::UP => PlacementDominoRotation::LEFT,
            PlacementDominoRotation::LEFT => PlacementDominoRotation::DOWN,
            PlacementDominoRotation::DOWN => PlacementDominoRotation::RIGHT,
            PlacementDominoRotation::RIGHT => PlacementDominoRotation::UP,
        }
    }
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
    domino_rotation: PlacementDominoRotation,
}

impl Gui {

    pub(crate) async fn new() -> Self {
        Self {
            assets: Assets::load().await,
            domino_rotation: PlacementDominoRotation::UP,
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
            self.draw_obj(self.assets.fetch_king_texture_by_turn(idx), 100.0, screen_height()*(3.0/4.0)-200.0+i*75.0, board_gui::SCORE_KING_SIZE);
        }

        // Draw colored borders within grid panes
        self.draw_color_border(board_gui::BLUE,   -1.0, -1.0);
        self.draw_color_border(board_gui::GREEN,   1.0, -1.0);
        self.draw_color_border(board_gui::RED,    -1.0,  1.0);
        self.draw_color_border(board_gui::YELLOW,  1.0,  1.0);

    }

    // Detects if R key is pressed and rotates the domino if so (only called during placement phase)
    pub(crate) fn check_r_key_pressed(&self) {
        if is_key_down(KeyCode::R) {
            self.domino_rotation.next();
            return;
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
                        return Some(draft.pick(i, cur_player.id()));
                    }
                }
            }
        }
        None
    }


    /// The overarching draw function. Called each frame of the game.
    pub(crate) fn draw(&self, pick_draft: &Draft, place_draft: &Draft, active_player_id: &usize, phase: &Phase, player_list: &[Player; 4], subturn_number: &u8) {
        let active_player = Gui::get_active_player(active_player_id, player_list);
        // assert!(active_player.picked().id() < 49, "You managed to click on a domino with id greater than 48. The id is {}", active_player.picked().id());
        let mut valid_draft_doms: [bool;4] = [true;4];
        clear_background(board_gui::BACKGROUND_COLOR);
        self.make_containers();
        self.add_advice_box(*active_player_id, phase); 
        self.draw_draft(pick_draft, draft_gui::PICK_DOMINO_X, valid_draft_doms); // unsure if valid_draft_doms should always be true for this line. If you get a weird error where the unpicked doms are not showing up, this line is the issue
        for temp_player in player_list {
            self.draw_domino_map(temp_player); // Draws the domino maps for each player regardless of if they are active
        }
        
        match phase {
            Phase::Placing => {
                valid_draft_doms = self.undraw_old_doms(subturn_number);
                self.draw_placing_textures_if_placing(active_player);
                self.draw_sockets(active_player);
            }
            Phase::Picking => {}
        }

        if !place_draft.is_null() {
            self.draw_draft(place_draft, draft_gui::PLACE_DOMINO_X, valid_draft_doms); //Written by Peter

        }
    }

    /// Returns the y coordinate of the top domino of the draft. Calculated based on screen height and draft size.
    fn top_draft_domino_y() -> f32 {
        // Half screen plus the two dominoes above the halfway point
        screen_height()/4.0 - 50.0 - (DRAFT_SIZE as f32/2.0) * draft_gui::VERT_OFFSET
    }

    fn draw_draft(&self, draft: &Draft, domino_x: f32, valid_doms: [bool;4]) { //Written by Peter

        if draft.is_null() {
            return;
        }

        let top_domino_y = Self::top_draft_domino_y();

        for (i, domino) in draft.iter().enumerate() {
            if valid_doms[i]{
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
    fn add_advice_box(&self, active_player_id: usize, phase: &Phase){
        //Gets the right text based on game phase
        let curr_advice: String;
        match phase {
            Phase::Placing => {
                curr_advice = String::from(PLACING_ADVICE);
            }
            &Phase::Picking => { // I have no idea why this a reference and the other is not but I dont want to touch it
                curr_advice = String::from(PICKING_ADVICE);
            }
        }
        //Draw text
        draw_multiline_text(&curr_advice, -10.0, screen_height()/2.0 - 75.0, 20.0, Some(0.3), WHITE);
        
        //Draw king of active player
        self.draw_obj(self.assets.fetch_king_texture_by_turn(active_player_id as u8), screen_width()/3.0-50.0, screen_height()/2.0, 30.0);
    }

    fn undraw_old_doms(&self, subturn_number: &u8) -> [bool;4]  {
        let id = *subturn_number;
        let mut valid_dominos: [bool;4] = [true;4];
        for i in 0..id {
            valid_dominos[(i.saturating_sub(1)) as usize] = false;
        }
        valid_dominos

    }

    fn draw_placing_textures_if_placing(&self, active_player: &Player){
        
        // get cursor coords
        let (mouse_x, mouse_y) = mouse_position();
        // get rotation in radians
        let rotation: f64;
        match self.domino_rotation {
            PlacementDominoRotation::UP => {rotation = 0.0;}
            PlacementDominoRotation::LEFT => {rotation = PI/2.0;}
            PlacementDominoRotation::DOWN => {rotation = PI;}
            PlacementDominoRotation::RIGHT => {rotation = PI*(3.0/2.0);}
        }
        // then draw domino, based on rotation enum. (Pressing 'r' cycles through the enum)
        let mut x_offset: f32 = -draft_gui::DOMINO_TILE_SIZE/2.0; //TODO: once picking logic is done, fine tune offsets
        let mut y_offset: f32 = -draft_gui::DOMINO_TILE_SIZE/2.0; //TODO: once picking logic is done, fine tune offsets
        draw_texture_ex(
            self.assets.fetch_domino_texture_by_id(active_player.picked().id()).unwrap(),
            mouse_x + x_offset,
            mouse_y + y_offset,
            WHITE, DrawTextureParams {
            dest_size: Some(Vec2::new(draft_gui::DOMINO_TILE_SIZE*2.0, draft_gui::DOMINO_TILE_SIZE)),
            rotation: rotation as f32,
            ..Default::default()
        }, );
        // then draw hand
        x_offset = -draft_gui::DOMINO_TILE_SIZE/2.0; //TODO: once picking logic is done, fine tune offsets
        y_offset = -draft_gui::DOMINO_TILE_SIZE/2.0; //TODO: once picking logic is done, fine tune offsets
        self.draw_obj(self.assets.fetch_hand(), mouse_x + x_offset, mouse_y + y_offset, draft_gui::DOMINO_TILE_SIZE);

        return;
    }

    fn draw_sockets(&self, active_player: &Player){
        // get the correct socket map based on the orientation enum
        let socket_map: &[[bool; 9]; 9];
        match self.domino_rotation {
            PlacementDominoRotation::UP => {
                socket_map = active_player.grid().up_map();
            }
            PlacementDominoRotation::LEFT => {
                socket_map = active_player.grid().up_map();
            }
            PlacementDominoRotation::DOWN => {
                socket_map = active_player.grid().up_map();
            }
            PlacementDominoRotation::RIGHT => {
                socket_map = active_player.grid().up_map();
            }
        }

        //calculate offset (find coord of middle of player's box)
        let offset: (f32, f32) = Gui::get_active_player_box_offset(active_player);
        for row in 0..socket_map[0].len(){
            for col in 0..socket_map.len(){
                if socket_map[row][col] {
                    self.draw_obj(self.assets.fetch_socket(), offset.0, offset.1, draft_gui::DOMINO_TILE_SIZE);
                }
            }
        }
        

        return;
    }

    //get active player obj here so i dont have to do this a gazillion times
    fn get_active_player<'a>(active_player_id: &usize, player_list: &'a [Player; 4]) -> &'a Player {
        let id = *active_player_id;
        player_list.get(id.saturating_sub(1)).expect(&format!("There was an error. active_player_id is {}", *active_player_id)) 
    }

    // gives the coords of the active player's box
    fn get_active_player_box_offset(active_player: &Player) -> (f32, f32) {
        let id = active_player.id();
        let mut x_offset: f32 = screen_width()*(2.0/3.0);
        let mut y_offset: f32 = screen_height()/2.0;
        match id {
            1 => {
                x_offset = x_offset - screen_width()/6.0;
                y_offset = y_offset - screen_height()/4.0;
                (x_offset, y_offset)
            }
            2 => {
                x_offset = x_offset + screen_width()/6.0;
                y_offset = y_offset - screen_height()/4.0;
                (x_offset, y_offset)
            }
            3 => {
                x_offset = x_offset - screen_width()/6.0;
                y_offset = y_offset + screen_height()/4.0;
                (x_offset, y_offset)
            }
            4 => {
                x_offset = x_offset + screen_width()/6.0;
                y_offset = y_offset + screen_height()/4.0;
                (x_offset, y_offset)
            }
            _ => {panic!("got a player id that was not 1-4 in get_active_player_box_offset()");}
        }
    }

    fn draw_domino_map(&self, active_player: &Player) {
        let domino_map: &Vec<GridDomino> = active_player.grid().domino_map();
        let offset: (f32, f32) = Gui::get_active_player_box_offset(active_player);
        let x_offset: f32 = offset.0; // this offset pinpoints the exact x_pos center of the active_player's colored box.
        let y_offset: f32 = offset.1; // this offset pinpoints the exact y_pos center of the active_player's colored box.
        assert_ne!(0, domino_map.len(), "The length of the domino map is 0, it ought to start at 1. len is: {}", domino_map.len());

        for grid_domino in domino_map {
            let x = x_offset - draft_gui::DOMINO_TILE_SIZE/2.0 + grid_multipliers_gui::X_MULTIPLIER * (*grid_domino.x() as f32);
            let y = y_offset - draft_gui::DOMINO_TILE_SIZE/2.0 + grid_multipliers_gui::X_MULTIPLIER * (*grid_domino.y() as f32);
            
            let rotation: f64 = *grid_domino.rotation();
            let texture_option: Option<&Texture2D> = self.assets.fetch_domino_texture_by_id(*grid_domino.domino_id() as u8);
            draw_texture_ex(
                texture_option.unwrap(),
                x,
                y,
                WHITE, DrawTextureParams {
                dest_size: Some(Vec2::new(draft_gui::DOMINO_TILE_SIZE, draft_gui::DOMINO_TILE_SIZE)),
                rotation: rotation as f32,
                ..Default::default()
            }, );

        }

    }

}
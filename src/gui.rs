use std::f64::consts::PI;
use macroquad::color::WHITE;
use macroquad::input::{is_mouse_button_pressed, MouseButton};
use macroquad::prelude::*;
use crate::assets::Assets;
use crate::components::{draft, grid, grid_domino};
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
    use crate::gui::draft_gui;

    pub(crate) const X_MULTIPLIER: f32 = draft_gui::DOMINO_TILE_SIZE;
    pub(crate) const Y_MULTIPLIER: f32 = draft_gui::DOMINO_TILE_SIZE;
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

pub enum PlacementDominoRotation {
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
    blue_offset: [f32;2], //map offsets
    green_offset: [f32;2], //map offsets
    red_offset: [f32;2], //map offsets
    yellow_offset: [f32;2], //map offsets
    curr_socket_locations: Vec<[f32; 6]> // [socket_row, socket_col, x_lower, x_upper, y_lower, y_upper]
}

impl Gui {

    pub(crate) async fn new() -> Self {
        Self {
            assets: Assets::load().await,
            domino_rotation: PlacementDominoRotation::UP,
            blue_offset: [0.0;2],
            green_offset: [0.0;2],
            red_offset: [0.0;2],
            yellow_offset: [0.0;2],
            curr_socket_locations: Vec::new(),

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
    pub(crate) fn check_r_key_pressed(&mut self) {
        if is_key_pressed(KeyCode::R) {
            self.domino_rotation = self.domino_rotation.next();
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

    pub(crate) fn picked_socket(cur_player: &Player, curr_orientation: &PlacementDominoRotation, curr_socket_vec: &Vec<[f32; 6]>) -> Option<GridDomino> {
        if !is_mouse_button_pressed(MouseButton::Left) { return None; }
        let (mx, my) = mouse_position();

        let rotation: f64;
        match curr_orientation {
            PlacementDominoRotation::UP => {rotation = 0.0;}
            PlacementDominoRotation::LEFT => {rotation = PI/2.0;}
            PlacementDominoRotation::DOWN => {rotation = PI;}
            PlacementDominoRotation::RIGHT => {rotation = PI*(3.0/2.0);}
        }

        for entry in curr_socket_vec {
            if mx > entry[2] && mx < entry[3] && my > entry[4] && my < entry[5] {
                println!("row: {}, col: {}", entry[0], entry[1]);
                //reflect the index accross the center
                
                let x_low:  &usize = cur_player.grid().dm_lower_x();
                let x_high: &usize = cur_player.grid().dm_upper_x();
                let y_low:  &usize = cur_player.grid().dm_lower_y();
                let y_high: &usize = cur_player.grid().dm_upper_y();
                let mut x_center: f32 = ((*x_high as f32) + (*x_low as f32))/2.0;
                let mut y_center: f32 = ((*y_high as f32) + (*y_low as f32))/2.0;
                let x = 2.0*x_center - entry[0];
                let y = 2.0*y_center - entry[1];
                
                return Some(GridDomino::new(x as u8, y as u8, cur_player.placing().id() as usize, rotation));
            }
        }
        None
    }


    /// The overarching draw function. Called each frame of the game.
    pub(crate) fn draw(&mut self, pick_draft: &Draft, place_draft: &Draft, active_player_id: &usize, phase: &Phase, player_list: &[Player; 4], subturn_number: &u8) {
        let active_player = Gui::get_active_player(subturn_number, player_list);
        // assert!(active_player.picked().id() < 49, "You managed to click on a domino with id greater than 48. The id is {}", active_player.picked().id());
        let mut valid_draft_doms: [bool;4] = [true;4];
        clear_background(board_gui::BACKGROUND_COLOR);
        self.make_containers();
        self.add_advice_box(*active_player_id, phase); 
        self.draw_draft(pick_draft, draft_gui::PICK_DOMINO_X, valid_draft_doms); // unsure if valid_draft_doms should always be true for this line. If you get a weird error where the unpicked doms are not showing up, this line is the issue
        for temp_player in player_list {
            self.update_offset(temp_player);
            self.draw_domino_map(temp_player); // Draws the domino maps for each player regardless of if they are active
        }
        
        match phase {
            Phase::Placing => {
                self.draw_sockets(active_player);
                valid_draft_doms = self.undraw_old_doms(subturn_number);
                self.draw_placing_textures_if_placing(active_player);
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
        // offsets
        let mut x_offset: f32 = -draft_gui::DOMINO_TILE_SIZE/2.0; //TODO: once picking logic is done, fine tune offsets
        let mut y_offset: f32 = -draft_gui::DOMINO_TILE_SIZE/2.0; //TODO: once picking logic is done, fine tune offsets
        // get rotation in radians
        let rotation: f64;
        match self.domino_rotation {
            PlacementDominoRotation::UP => {rotation = 0.0;}
            PlacementDominoRotation::LEFT => {rotation = PI/2.0; x_offset = x_offset - draft_gui::DOMINO_TILE_SIZE/2.0; y_offset = y_offset + draft_gui::DOMINO_TILE_SIZE/2.0}
            PlacementDominoRotation::DOWN => {rotation = PI; x_offset = x_offset - draft_gui::DOMINO_TILE_SIZE}
            PlacementDominoRotation::RIGHT => {rotation = PI*(3.0/2.0); x_offset = x_offset - draft_gui::DOMINO_TILE_SIZE/2.0; y_offset = y_offset - draft_gui::DOMINO_TILE_SIZE/2.0}
        }
        // then draw domino, based on rotation enum. (Pressing 'r' cycles through the enum)
        draw_texture_ex(
            self.assets.fetch_domino_texture_by_id(active_player.placing().id()).unwrap(),
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

    fn draw_sockets(&mut self, active_player: &Player){
        // get the correct socket map based on the orientation enum
        let socket_map: &[[bool; 9]; 9];
        match self.domino_rotation {
            PlacementDominoRotation::UP => {
                socket_map = active_player.grid().up_map();
            }
            PlacementDominoRotation::LEFT => {
                socket_map = active_player.grid().left_map();
            }
            PlacementDominoRotation::DOWN => {
                socket_map = active_player.grid().down_map();
            }
            PlacementDominoRotation::RIGHT => {
                socket_map = active_player.grid().right_map();
            }
        }

        // calculate offset (copy the offset out so we don't hold an immutable borrow of self)
        let map_offset = *self.fetch_offset(active_player.id());

        // reset stored sockets for this frame
        self.curr_socket_locations.clear();

        for row in 0..socket_map[0].len(){
            for col in 0..socket_map.len(){
                // print!("{}\t", socket_map[row][col]);
                if socket_map[row][col] {   
                    let x_lower = map_offset[0] - (8.0 - row as f32)*grid_multipliers_gui::X_MULTIPLIER;
                    let y_lower = map_offset[1] - (8.0 - col as f32)*grid_multipliers_gui::Y_MULTIPLIER;
                    let x_upper = x_lower + draft_gui::DOMINO_TILE_SIZE;
                    let y_upper = y_lower + draft_gui::DOMINO_TILE_SIZE;
                    self.curr_socket_locations.push([row as f32, col as f32, x_lower, x_upper, y_lower, y_upper]);
                    self.draw_obj(self.assets.fetch_socket(),
                    map_offset[0] - (8.0 - row as f32)*grid_multipliers_gui::X_MULTIPLIER, 
                    map_offset[1] - (8.0 - col as f32)*grid_multipliers_gui::Y_MULTIPLIER, 
                    draft_gui::DOMINO_TILE_SIZE);
                }
            }
            // print!("\n")
        }
        //panic!("End of testing");
        

        return;
    }

    //get active player obj here so i dont have to do this a gazillion times
    fn get_active_player<'a>(active_player_id: &u8, player_list: &'a [Player; 4]) -> &'a Player {
        let id = (*active_player_id).saturating_sub(1);
        player_list.get(id as usize).expect(&format!("There was an error. active_player_id is {}", *active_player_id)) 
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
        let map_offset: &[f32; 2] = self.fetch_offset(active_player.id());
        assert_ne!(0, domino_map.len(), "The length of the domino map is 0, it ought to start at 1. len is: {}", domino_map.len());
        
        for grid_domino in domino_map {
            let x = map_offset[0] - grid_multipliers_gui::X_MULTIPLIER * (*grid_domino.x() as f32);
            let y = map_offset[1] - grid_multipliers_gui::Y_MULTIPLIER * (*grid_domino.y() as f32);
            let rotation: f64 = *grid_domino.rotation();
            let texture_option: Option<&Texture2D> = self.assets.fetch_domino_texture_by_id(*grid_domino.domino_id() as u8);
            if *grid_domino.domino_id() == 49 {
                draw_texture_ex(
                texture_option.unwrap(),
                x,
                y,
                WHITE, DrawTextureParams {
                dest_size: Some(Vec2::new(draft_gui::DOMINO_TILE_SIZE, draft_gui::DOMINO_TILE_SIZE)),
                rotation: rotation as f32,
                ..Default::default()
            }, );
            } else {
                draw_texture_ex(
                texture_option.unwrap(),
                x,
                y,
                WHITE, DrawTextureParams {
                dest_size: Some(Vec2::new(draft_gui::DOMINO_TILE_SIZE*2.0, draft_gui::DOMINO_TILE_SIZE)),
                rotation: rotation as f32,
                ..Default::default()
            }, );
            }
            // TESTING:::::
            // for i in 0..9 {
            //     for j in 0..9 {
            //         print!("{:?}\n",active_player.grid().tile_map()[i][j]);
            //     }
            //     println!("");
            // }
            // END TESTING:::::
            

        }

    }

    fn fetch_offset(&self, id: u8) -> &[f32; 2] {
        match id {
            1 => {&self.blue_offset}
            2 => {&self.green_offset}
            3 => {&self.red_offset}
            4 => {&self.yellow_offset}
            _ => {panic!("you called fetch_offset with an id not corresponding to any colors")}
        }
    }

    fn update_offset(&mut self, active_player: &Player) {
        let offset: (f32, f32) = Gui::get_active_player_box_offset(active_player); // box offset... still needs the map offset

        let x_low:  &usize = active_player.grid().dm_lower_x();
        let x_high: &usize = active_player.grid().dm_upper_x();
        let y_low:  &usize = active_player.grid().dm_lower_y();
        let y_high: &usize = active_player.grid().dm_upper_y();
        let mut x_center: f32 = ((*x_high as f32) + (*x_low as f32) - 1.0)/2.0;
        let mut y_center: f32 = ((*y_high as f32) + (*y_low as f32) - 1.0)/2.0;
        x_center = offset.0 + x_center * grid_multipliers_gui::X_MULTIPLIER;
        y_center = offset.1 + y_center * grid_multipliers_gui::Y_MULTIPLIER;

        match active_player.id() {
            1 => {self.blue_offset[0] = x_center; self.blue_offset[1] = y_center; }
            2 => {self.green_offset[0] = x_center; self.green_offset[1] = y_center; }
            3 => {self.red_offset[0] = x_center; self.red_offset[1] = y_center; }
            4 => {self.yellow_offset[0] = x_center; self.yellow_offset[1] = y_center; }
            _ => {panic!("you called fetch_offset with an id not corresponding to any colors")}
        } 
        return;
    }

    pub(crate) fn domino_rotation(&self) -> &PlacementDominoRotation {&self.domino_rotation}
    pub(crate) fn get_socket_vec(&self) -> &Vec<[f32; 6]> {&self.curr_socket_locations}
}
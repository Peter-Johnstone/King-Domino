use crate::components::domino::Domino;
use crate::components::tile::Tile;
use crate::components::tile::Types;
use crate::components::grid_domino::GridDomino;
use crate::gui::rotation;

pub enum BuildRotation{
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl BuildRotation{
    fn next(&self) -> Self {
        match *self {
            BuildRotation::UP => BuildRotation::DOWN,
            BuildRotation::DOWN => BuildRotation::LEFT,
            BuildRotation::LEFT => BuildRotation::RIGHT,
            BuildRotation::RIGHT => BuildRotation::UP, // Cycles back to North
        }
    }
}

#[derive(Default, Clone)]
pub(crate) struct Grid
{
    // Does ALL computational heavy lifting
    tile_map: [[Tile; 9]; 9],

    // Probably want another list of maps strictly for bot use
    bot_maps: Vec<[[Tile; 9]; 9]>,

    // Idea: maybe the maps only contain sockets? Like 1 if exists 0 if not. sockets are buttons overlayed on domino map
    // list of possible tile_maps for up
    up_map: [[u8; 5]; 5],//Vec<Vec<Vec<Tile>>>,
    // list of possible tile_maps for down
    down_map: [[u8; 5]; 5],
    // list of possible tile_maps for left
    left_map: [[u8; 5]; 5],
    // list of possible tile_maps for right
    right_map: [[u8; 5]; 5],
    //Does not need to be computationally efficient or useful at ALL. Only purpose is for GUI
    domino_map: Vec<GridDomino>,

    // Bounds used for centering the domino_grid for the gui. dm = Domino Map
    dm_lower_x: usize,
    dm_upper_x: usize,
    dm_lower_y: usize,
    dm_upper_y: usize,
}

impl Grid
{
    pub(crate) fn new() -> Self {
        let starting_map = Self::make_starting_map();
        Self {
            tile_map: starting_map, //vec![vec![Tile::new(Types::Castle, 0)]],
            bot_maps: Default::default(),
            up_map: [[0u8; 5]; 5],
            down_map: [[0u8; 5]; 5],
            left_map: [[0u8; 5]; 5],
            right_map: [[0u8; 5]; 5],
            domino_map: vec![GridDomino::new(0,0,99,0.0)],

            dm_lower_x: 0,
            dm_upper_x: 0,
            dm_lower_y: 0,
            dm_upper_y: 0,
        }
    }

    // Tell this function a domino and it will generate maps for up, down, left, and right
    pub(crate) fn build_maps(&mut self, new_domino: Domino) {
        let mut rotation = BuildRotation::UP;
        let anchor_type = new_domino.get_tile_type(1);
        let second_type = new_domino.get_tile_type(2);

        for _ in 0..4 { // Searching all 4 directions
            //For each tile in our map...
            let rows = self.tile_map.len();
            for i in 0..rows {
                let cols = self.tile_map[i].len();
                for j in 0..cols {
                    let curr_type = self.tile_map[i][j].get_type(); // short-lived borrow
                    if curr_type == anchor_type {
                        self.anchor_match(&rotation, i, j); // mutable borrow allowed now
                    } else if curr_type == second_type {
                        self.second_match(&rotation,i , j);// handle second-type matches
                    }
                }
            }

            rotation = rotation.next(); //cycle to next rotation of the domino
        }
                
    }

    fn anchor_match(&mut self, rotation: &BuildRotation, i: usize, j: usize) {
        match *rotation {
            BuildRotation::UP => { //UP means the tile is "right side up". Rotation = 0
                if self.tile_map[i][j+1].get_type() == Types::Null && self.tile_map[i+1][j+1].get_type() == Types::Null {
                    if ((i+1) - self.dm_lower_x) <= 5 && ((j+1) - self.dm_lower_y) <= 5 {self.up_map[i][j+1] = 1}
                }
                if self.tile_map[i+1][j].get_type() == Types::Null && self.tile_map[i+2][j].get_type() == Types::Null {
                    if ((i+2) - self.dm_lower_x) <= 5 {self.up_map[i+1][j] = 1}
                }
                if self.tile_map[i][j-1].get_type() == Types::Null && self.tile_map[i+1][j-1].get_type() == Types::Null {
                    if ((i+1) - self.dm_lower_x) <= 5 && (self.dm_upper_y - (j-1)) <= 5 {self.up_map[i][j-1] = 1}
                }
            }
            BuildRotation::DOWN => { //Tile is upside down. Rotation = pi radians
                if self.tile_map[i][j+1].get_type() == Types::Null && self.tile_map[i-1][j+1].get_type() == Types::Null {
                    if self.dm_upper_x - ((i-1)) <= 5 && ((j+1) - self.dm_lower_y) <= 5 {self.down_map[i][j+1] = 1}
                }
                if self.tile_map[i-1][j].get_type() == Types::Null && self.tile_map[i-2][j].get_type() == Types::Null {
                    if self.dm_upper_x - (i-2) <= 5 {self.down_map[i-1][j] = 1}
                }
                if self.tile_map[i][j-1].get_type() == Types::Null && self.tile_map[i-1][j-1].get_type() == Types::Null {
                    if self.dm_upper_x - ((i-1)) <= 5 && (self.dm_upper_y - (j-1)) <= 5 {self.down_map[i][j-1] = 1}
                }
            }
            BuildRotation::LEFT => { //Rotation is pi/2 radians
                if self.tile_map[i+1][j].get_type() == Types::Null && self.tile_map[i+1][j+1].get_type() == Types::Null {
                    if ((i+1) - self.dm_lower_x) <= 5 && ((j+1) - self.dm_lower_y) <= 5 {self.left_map[i+1][j] = 1}
                }
                if self.tile_map[i][j+1].get_type() == Types::Null && self.tile_map[i][j+2].get_type() == Types::Null {
                    if ((j+2) - self.dm_lower_y) <= 5 {self.left_map[i][j+1] = 1}
                }
                if self.tile_map[i-1][j].get_type() == Types::Null && self.tile_map[i-1][j+1].get_type() == Types::Null {
                    if self.dm_upper_x - ((i-1)) <= 5 && ((j+1) - self.dm_lower_y) <= 5 {self.left_map[i-1][j] = 1}
                }
            }
            BuildRotation::RIGHT => { //Rotation is 3pi/2 radians
                if self.tile_map[i+1][j].get_type() == Types::Null && self.tile_map[i+1][j-1].get_type() == Types::Null {
                    if ((i+1) - self.dm_lower_x) <= 5 && (self.dm_upper_y - (j-1)) <= 5 {self.right_map[i+1][j] = 1}
                }
                if self.tile_map[i][j-1].get_type() == Types::Null && self.tile_map[i][j-2].get_type() == Types::Null {
                    if (self.dm_lower_y - (j-2)) <= 5 {self.right_map[i][j-1] = 1}
                }
                if self.tile_map[i-1][j].get_type() == Types::Null && self.tile_map[i-1][j-1].get_type() == Types::Null {
                    if self.dm_upper_x - ((i-1)) <= 5 && (self.dm_upper_y - (j-1)) <= 5 {self.right_map[i-1][j] = 1}
                }
            }
        }
    }
    
    // This logic is quite complex. Lets see if I can manage not to mess it up
    // TODO: Add the logic for this. I think can do this quite quickly tomorrow
    fn second_match (&mut self, rotation: &BuildRotation, i: usize, j: usize){
        match *rotation {
            BuildRotation::UP => {

            }
            BuildRotation::DOWN => {

            }
            BuildRotation::LEFT => {

            }
            BuildRotation::RIGHT => {

            }
        }
    }

    //These ought 4 fns to be called upon by the gui
    pub(crate) fn get_up_map(&self) -> &[[u8; 5]; 5] {
        &self.up_map
    }
    pub(crate) fn get_down_map(&self) -> &[[u8; 5]; 5] {
        &self.down_map
    }
    pub(crate) fn get_left_map(&self) -> &[[u8; 5]; 5] {
        &self.left_map
    }
    pub(crate) fn get_right_map(&self) -> &[[u8; 5]; 5] {
        &self.right_map
    }

    //User clicks a socket. This should be called by the gui
    pub(crate) fn position_selected(&self, x: u8, y: u8, id: usize, domino_rotation: f32){
        // TODO: Implement

        // Pseudocode:
        // self.domino_map.push(GridDomino::new(x, y, id, domino_rotation))
        // Update x,y upper,lower bounds 
    }

    //Returns true if the player has no room left on their 5x5 grid to place a new tile.
    //maybe check if build maps cannot build any maps?
    pub(crate) fn has_no_room_left(&self) -> bool {
        // TODO: impment. not a "now" problem though. yet.
        false
    }

    fn make_starting_map() -> [[Tile;9];9]{
        let mut temp_map: [[Tile;9];9] = [[Tile::default();9];9];
        temp_map[4][4] = Tile::new(Types::Castle, 0);
        temp_map
    }
}
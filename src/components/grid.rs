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
    up_map: [[bool; 9]; 9],//Vec<Vec<Vec<Tile>>>,
    // list of possible tile_maps for down
    down_map: [[bool; 9]; 9],
    // list of possible tile_maps for left
    left_map: [[bool; 9]; 9],
    // list of possible tile_maps for right
    right_map: [[bool; 9]; 9],
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
            up_map: [[false; 9]; 9],
            down_map: [[false; 9]; 9],
            left_map: [[false; 9]; 9],
            right_map: [[false; 9]; 9],
            domino_map: vec![GridDomino::new(0,0,99,0.0)],

            dm_lower_x: 0,
            dm_upper_x: 0,
            dm_lower_y: 0,
            dm_upper_y: 0,
        }
    }

    // Tell this function a domino and it will generate maps for up, down, left, and right
    pub(crate) fn build_maps(&mut self, new_domino: Domino) -> bool {
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
                        self.second_match(&rotation, i, j);// handle second-type matches
                    }
                }
            }
            rotation = rotation.next(); //cycle to next rotation of the domino
        }

        //empty the bot_maps vector
        self.bot_maps.clear();
        self.build_bot_maps(rotation, &new_domino);

        //If there are no bot maps, then the player has no room to place any tiles left!
        if 0 == self.bot_maps.len() {
            return false
        }
        true
                
    }

    fn anchor_match(&mut self, rotation: &BuildRotation, i: usize, j: usize) {
        if j == 0 || i ==0 || j == 8 || i == 8 {

        }
        match *rotation {
            BuildRotation::UP => { //UP means the tile is "right side up". Rotation = 0
                if i != 8 && j != 8 && self.tile_map[i][j+1].get_type() == Types::Null && self.tile_map[i+1][j+1].get_type() == Types::Null {
                    if ((i+1).saturating_sub(self.dm_lower_x)) <= 5 && ((j+1).saturating_sub(self.dm_lower_y)) <= 5 {self.up_map[i][j+1] = true}
                }
                if i < 7 && self.tile_map[i+1][j].get_type() == Types::Null && self.tile_map[i+2][j].get_type() == Types::Null {
                    if ((i+2).saturating_sub(self.dm_lower_x)) <= 5 {self.up_map[i+1][j] = true}
                }
                if i != 8 && j != 0 && self.tile_map[i][j.saturating_sub(1)].get_type() == Types::Null && self.tile_map[i+1][j.saturating_sub(1)].get_type() == Types::Null {
                    if ((i+1).saturating_sub(self.dm_lower_x)) <= 5 && (self.dm_upper_y.saturating_sub(j.saturating_sub(1))) <= 5 {self.up_map[i][j.saturating_sub(1)] = true}
                }
            }
            BuildRotation::DOWN => { //Tile is upside down. Rotation = pi radians
                if i != 0 && j != 8 && self.tile_map[i][j+1].get_type() == Types::Null && self.tile_map[i.saturating_sub(1)][j+1].get_type() == Types::Null {
                    if self.dm_upper_x.saturating_sub(i.saturating_sub(1)) <= 5 && ((j+1).saturating_sub(self.dm_lower_y)) <= 5 {self.down_map[i][j+1] = true}
                }
                if i > 1 && self.tile_map[i.saturating_sub(1)][j].get_type() == Types::Null && self.tile_map[i.saturating_sub(2)][j].get_type() == Types::Null {
                    if self.dm_upper_x.saturating_sub(i.saturating_sub(2)) <= 5 {self.down_map[i.saturating_sub(1)][j] = true}
                }
                if i != 0 && j != 0 && self.tile_map[i][j.saturating_sub(1)].get_type() == Types::Null && self.tile_map[i.saturating_sub(1)][j.saturating_sub(1)].get_type() == Types::Null {
                    if self.dm_upper_x.saturating_sub(i.saturating_sub(1)) <= 5 && (self.dm_upper_y.saturating_sub(j.saturating_sub(1))) <= 5 {self.down_map[i][j.saturating_sub(1)] = true}
                }
            }
            BuildRotation::LEFT => { //Rotation is pi/2 radians
                if i != 8 && j != 8 && self.tile_map[i+1][j].get_type() == Types::Null && self.tile_map[i+1][j+1].get_type() == Types::Null {
                    if ((i+1).saturating_sub(self.dm_lower_x)) <= 5 && ((j+1).saturating_sub(self.dm_lower_y)) <= 5 {self.left_map[i+1][j] = true}
                }
                if j < 7 && self.tile_map[i][j+1].get_type() == Types::Null && self.tile_map[i][j+2].get_type() == Types::Null {
                    if ((j+2).saturating_sub(self.dm_lower_y)) <= 5 {self.left_map[i][j+1] = true}
                }
                if i != 0 && j != 8 && self.tile_map[i.saturating_sub(1)][j].get_type() == Types::Null && self.tile_map[i.saturating_sub(1)][j+1].get_type() == Types::Null {
                    if self.dm_upper_x.saturating_sub(i.saturating_sub(1)) <= 5 && ((j+1).saturating_sub(self.dm_lower_y)) <= 5 {self.left_map[i.saturating_sub(1)][j] = true}
                }
            }
            BuildRotation::RIGHT => { //Rotation is 3pi/2 radians
                if i != 8 && j != 8 && self.tile_map[i+1][j].get_type() == Types::Null && self.tile_map[i+1][j.saturating_sub(1)].get_type() == Types::Null {
                    if ((i+1).saturating_sub(self.dm_lower_x)) <= 5 && (self.dm_upper_y.saturating_sub(j.saturating_sub(1))) <= 5 {self.right_map[i+1][j] = true}
                }
                if j > 1 && self.tile_map[i][j.saturating_sub(1)].get_type() == Types::Null && self.tile_map[i][j.saturating_sub(2)].get_type() == Types::Null {
                    if (self.dm_lower_y.saturating_sub(j.saturating_sub(2))) <= 5 {self.right_map[i][j.saturating_sub(1)] = true}
                }
                if i != 0 && j != 0 && self.tile_map[i.saturating_sub(1)][j].get_type() == Types::Null && self.tile_map[i.saturating_sub(1)][j.saturating_sub(1)].get_type() == Types::Null {
                    if self.dm_upper_x.saturating_sub(i.saturating_sub(1)) <= 5 && (self.dm_upper_y.saturating_sub(j.saturating_sub(1))) <= 5 {self.right_map[i.saturating_sub(1)][j] = true}
                }
            }
        }
    }
    
    // This logic is quite complex. Lets see if I can manage not to mess it up
    // TODO: Add the logic for this. I think can do this quite quickly tomorrow
    fn second_match (&mut self, rotation: &BuildRotation, i: usize, j: usize){
        match *rotation {
            BuildRotation::UP => {
                if i != 0 && j != 8 && self.tile_map[i][j+1].get_type() == Types::Null && self.tile_map[i.saturating_sub(1)][j+1].get_type() == Types::Null {
                    if self.dm_upper_x.saturating_sub(i.saturating_sub(1)) <= 5 && ((j+1).saturating_sub(self.dm_lower_y)) <= 5 {self.up_map[i][j+1] = true}
                }
                if i > 1 && self.tile_map[i.saturating_sub(1)][j].get_type() == Types::Null && self.tile_map[i.saturating_sub(2)][j].get_type() == Types::Null {
                    if self.dm_upper_x.saturating_sub(i.saturating_sub(2)) <= 5 {self.up_map[i.saturating_sub(1)][j] = true}
                }
                if i != 0 && j != 0 && self.tile_map[i][j.saturating_sub(1)].get_type() == Types::Null && self.tile_map[i.saturating_sub(1)][j.saturating_sub(1)].get_type() == Types::Null {
                    if self.dm_upper_x.saturating_sub(i.saturating_sub(1)) <= 5 && (self.dm_upper_y.saturating_sub(j.saturating_sub(1))) <= 5 {self.up_map[i][j.saturating_sub(1)] = true}
                }
            }
            BuildRotation::DOWN => {
                if i != 8 && j != 8 && self.tile_map[i][j+1].get_type() == Types::Null && self.tile_map[i+1][j+1].get_type() == Types::Null {
                    if ((i+1).saturating_sub(self.dm_lower_x)) <= 5 && ((j+1).saturating_sub(self.dm_lower_y)) <= 5 {self.down_map[i][j+1] = true}
                }
                if i < 7 && self.tile_map[i+1][j].get_type() == Types::Null && self.tile_map[i+2][j].get_type() == Types::Null {
                    if ((i+2).saturating_sub(self.dm_lower_x)) <= 5 {self.down_map[i+1][j] = true}
                }
                if i != 8 && j != 0 && self.tile_map[i][j.saturating_sub(1)].get_type() == Types::Null && self.tile_map[i+1][j.saturating_sub(1)].get_type() == Types::Null {
                    if ((i+1).saturating_sub(self.dm_lower_x)) <= 5 && (self.dm_upper_y.saturating_sub(j.saturating_sub(1))) <= 5 {self.down_map[i][j.saturating_sub(1)] = true}
                }
            }
            BuildRotation::LEFT => {
                if i != 8 && j != 8 && self.tile_map[i+1][j].get_type() == Types::Null && self.tile_map[i+1][j.saturating_sub(1)].get_type() == Types::Null {
                    if ((i+1).saturating_sub(self.dm_lower_x)) <= 5 && (self.dm_upper_y.saturating_sub(j.saturating_sub(1))) <= 5 {self.left_map[i+1][j] = true}
                }
                if j > 1 && self.tile_map[i][j.saturating_sub(1)].get_type() == Types::Null && self.tile_map[i][j.saturating_sub(2)].get_type() == Types::Null {
                    if (self.dm_lower_y.saturating_sub(j.saturating_sub(2))) <= 5 {self.left_map[i][j.saturating_sub(1)] = true}
                }
                if i != 0 && j != 0 && self.tile_map[i.saturating_sub(1)][j].get_type() == Types::Null && self.tile_map[i.saturating_sub(1)][j.saturating_sub(1)].get_type() == Types::Null {
                    if self.dm_upper_x.saturating_sub(i.saturating_sub(1)) <= 5 && (self.dm_upper_y.saturating_sub(j.saturating_sub(1))) <= 5 {self.left_map[i.saturating_sub(1)][j] = true}
                }
            }
            BuildRotation::RIGHT => {
                if i != 8 && j != 8 && self.tile_map[i+1][j].get_type() == Types::Null && self.tile_map[i+1][j+1].get_type() == Types::Null {
                    if ((i+1).saturating_sub(self.dm_lower_x)) <= 5 && ((j+1).saturating_sub(self.dm_lower_y)) <= 5 {self.right_map[i+1][j] = true}
                }
                if j < 7 && self.tile_map[i][j+1].get_type() == Types::Null && self.tile_map[i][j+2].get_type() == Types::Null {
                    if ((j+2).saturating_sub(self.dm_lower_y)) <= 5 {self.right_map[i][j+1] = true}
                }
                if i != 0 && j != 8 && self.tile_map[i.saturating_sub(1)][j].get_type() == Types::Null && self.tile_map[i.saturating_sub(1)][j+1].get_type() == Types::Null {
                    if self.dm_upper_x.saturating_sub(i.saturating_sub(1)) <= 5 && ((j+1).saturating_sub(self.dm_lower_y)) <= 5 {self.right_map[i.saturating_sub(1)][j] = true}
                }
            }
        }
    }

    // Builds a new tile map for each socket on each up_map, down_map, left_map, right_map
    // This function is what builds self.bot_maps: Vec<[[Tile;9];9]>
    fn build_bot_maps(&mut self, mut rotation: BuildRotation, new_domino: &Domino){
        //first make a temp 9x9 array with the merged results for each other map
        let anchor_tile = new_domino.get_tile(1);
        let second_tile = new_domino.get_tile(2);
        for _ in 0..4 {
            for i in 0..self.tile_map.len() {
                for j in 0..self.tile_map[0].len() {
                    let mut temp_map: [[Tile;9];9] = self.tile_map.clone();
                    match rotation {
                        BuildRotation::UP => { 
                            if self.up_map[i][j] {
                                temp_map[i][j] = anchor_tile;
                                temp_map[i+1][j] = second_tile;
                            }
                        }
                        BuildRotation::DOWN => { 
                            if self.down_map[i][j] {
                                temp_map[i][j] = anchor_tile;
                                temp_map[i-1][j] = second_tile;
                            }
                        }
                        BuildRotation::LEFT => { 
                            if self.left_map[i][j] {
                                temp_map[i][j] = anchor_tile;
                                temp_map[i][j+1] = second_tile;
                            }
                        }
                        BuildRotation::RIGHT => {
                            if self.right_map[i][j] {
                                temp_map[i][j] = anchor_tile;
                                temp_map[i][j-1] = second_tile;
                            }
                        }
                    }
                    self.bot_maps.push(temp_map);
                }
            }
            rotation = rotation.next();
        }
    }

    //These ought 4 fns to be called upon by the gui
    pub(crate) fn get_up_map(&self) -> &[[bool; 9]; 9] {
        &self.up_map
    }
    pub(crate) fn get_down_map(&self) -> &[[bool; 9]; 9] {
        &self.down_map
    }
    pub(crate) fn get_left_map(&self) -> &[[bool; 9]; 9] {
        &self.left_map
    }
    pub(crate) fn get_right_map(&self) -> &[[bool; 9]; 9] {
        &self.right_map
    }

    //Get the bot maps
    pub(crate) fn get_bot_maps(&self) -> &Vec<[[Tile; 9]; 9]> {
        &self.bot_maps
    }

    //User clicks a socket. This should be called by the gui
    pub(crate) fn position_selected(&self, x: u8, y: u8, id: usize, domino_rotation: f32){
        // TODO: Implement

        // Pseudocode:
        // self.domino_map.push(GridDomino::new(x, y, id, domino_rotation))
        // Update x,y upper,lower bounds 
    }

    fn make_starting_map() -> [[Tile;9];9]{
        let mut temp_map: [[Tile;9];9] = [[Tile::default();9];9];
        temp_map[4][4] = Tile::new(Types::Castle, 0);
        temp_map
    }
}
use crate::components::domino::Domino;
use crate::components::tile::Tile;
use crate::components::tile::Types;
use crate::components::grid_domino::Grid_domino;
use crate::gui::rotation;


#[derive(Default, Clone)]
pub(crate) struct Grid
{
    // Does ALL computational heavy lifting
    tile_map: Vec<Vec<Tile>>,
    // list of possible tile_maps for up
    up_maps: Vec<Vec<Vec<Tile>>>,
    // list of possible tile_maps for down
    down_maps: Vec<Vec<Vec<Tile>>>,
    // list of possible tile_maps for left
    left_maps: Vec<Vec<Vec<Tile>>>,
    // list of possible tile_maps for right
    right_maps: Vec<Vec<Vec<Tile>>>,
    //Does not need to be computationally efficient or useful at ALL. Only purpose is for GUI
    domino_map: Vec<Grid_domino>,

    // Bounds used for centering the domino_grid for the gui. dm = Domino Map
    dm_lower_x: u8,
    dm_upper_x: u8,
    dm_lower_y: u8,
    dm_upper_y: u8,
}

impl Grid
{
    pub(crate) fn new() -> Self {
        Self {
            tile_map: vec![vec![Tile::new(Types::Castle, 0)]],
            up_maps: Default::default(),
            down_maps: Default::default(),
            left_maps: Default::default(),
            right_maps: Default::default(),
            domino_map: vec![Grid_domino::new(0,0,99,0.0)],

            dm_lower_x: 0,
            dm_upper_x: 0,
            dm_lower_y: 0,
            dm_upper_y: 0,
        }
    }

    // Tell this function a domino and it will generate maps for up, down, left, and right
    pub(crate) fn build_maps(&self, new_domino: Domino) {
        //TODO: implement
    }

    pub(crate) fn get_up_maps(&self) -> &Vec<Vec<Vec<Tile>>> {
        &self.up_maps
    }
    pub(crate) fn get_down_maps(&self) -> &Vec<Vec<Vec<Tile>>> {
        &self.down_maps
    }
    pub(crate) fn get_left_maps(&self) -> &Vec<Vec<Vec<Tile>>> {
        &self.left_maps
    }
    pub(crate) fn get_right_maps(&self) -> &Vec<Vec<Vec<Tile>>> {
        &self.right_maps
    }

    //User clicks a socket. This should be called by the gui
    pub(crate) fn position_selected(&self, domino_rotation: rotation, socket_id: usize){
        // TODO: Implement

        //Pseudocode:
        //
    }

    //Returns true if the player has no room left on their 5x5 grid to place a new tile.
    //maybe check if build maps cannot build any maps?
    pub(crate) fn has_no_room_left(&self) -> bool {
        // TODO: impment. not a "now" problem though. yet.
        false
    }
}
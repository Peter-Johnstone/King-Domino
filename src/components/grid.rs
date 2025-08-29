use crate::components::tile::Tile;

#[derive(Default, Clone, Copy)]
pub(crate) struct Grid
{
    map: [[Tile; 9]; 9],
    mappable: [[bool; 9]; 9],
    castle_coords: [u8; 2],
}

impl Grid
{
    pub(crate) fn empty() -> Grid {
        Grid{
            map: [[Tile::default(); 9]; 9],
            mappable: [[true; 9]; 9],
            castle_coords: [4,4]
        }                                
    }

    pub(crate) fn add_tile(&mut self, new_tile: Tile, row: usize, col: usize){
        if (self.map[row][col] != Tile::default()){
            println!("Error")
        }
        self.map[row][col] = new_tile;
    }

    pub(crate) fn get_domain() -> [[u8; 7]; 7] {
        [[0; 7]; 7]
    }


    pub(crate) fn is_full(&self) -> bool {
        false
    }
}
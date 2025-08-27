use crate::tile::Tile;

#[derive(Clone)]
pub(crate) struct Grid
{
    map: [[Tile; 9]; 9],
    mapable: [[bool; 9]; 9],
    castle_coords: [u8; 2],
}

#[derive(Default)]
impl Grid 
{
    pub(crate) fn new() -> Grid {
        Grid{
            map: [[Tile::Default; 9]; 9],
            mappable: [[true; 9]; 9],
            castle_coords: [4,4]
        }                                
    }

    pub(crate) fn add_tile(&self, new_tile: Tile, row: u8, col: u8){
        if (self.map[row][col] != Tile::Default){
            printf("u got an error");
        }
        self.map[row][col] = new_tile;
    }

    pub(crate) fn get_domain() -> [[u8; 7], 7] {
        for (u8 i = 0; i < mapable)
    }



}
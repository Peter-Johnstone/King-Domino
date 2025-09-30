use std::cmp::Ordering;
use crate::components::tile::Tile;
use crate::components::tile::Types;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Domino
{
    id: u8,
    tile1: Tile,
    tile2: Tile,
}



impl Ord for Domino {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Domino {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other)) // delegate to Ord
    }
}


const NULL_ID: u8 = 100;

impl Domino
{

    pub(crate) const fn new(id: u8, tile1: Tile, tile2: Tile) -> Self {
        Self {
            id,
            tile1,
            tile2
        }
    }

    /// Checks if the current domino is valid
    pub(crate) fn is_null(&self) -> bool {
        self.id == NULL_ID
    }

    /// returns a null domino
    pub(crate) fn null() -> Self {
        Self {
            id: NULL_ID,
            tile1: Tile::default(),
            tile2: Tile::default(),
        }
    }



    /// Grabs the id of the domino
    pub(crate) fn id(&self) -> u8 { self.id }


    // takes id (1 or 2) and returns the tile type of tile1 or tile2 respectivly
    pub(crate) fn get_tile_type(&self, id: usize) -> Types{
        match id {
            1 => {self.tile1.get_type()}
            2 => {self.tile2.get_type()}
            _ => {panic!("Invalid id passed to get_tile_type: expected 1 or 2, got {}", id)}
        }
    }

    pub(crate) fn get_tile(&self, id: usize) -> Tile{
        match id {
            1 => {self.tile1}
            2 => {self.tile2}
            _ => {panic!("You tried to get a tile that did not exist, you need to use 1 or 2 as a parameter")}
        }
    }
    

}
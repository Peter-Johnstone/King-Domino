use std::cmp::Ordering;
use crate::deck::DOMINO_SET;
use crate::tile::Tile;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Domino
{
    selectable: bool,
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

    fn what_is_my_id(&self) -> u8 {
        self.id
    }

    fn fetch(id: u8) -> Domino {
        DOMINO_SET[id as usize].clone()
    }

    /// returns a null domino
    pub(crate) fn null() -> Domino {
        Domino {
            selectable: false,
            id: NULL_ID,
            tile1: Tile::default(),
            tile2: Tile::default(),
        }
    }
    pub(crate) fn nullify(&mut self) {
        self.selectable = false;
        self.id = NULL_ID;
        self.tile1 = Tile::default();
        self.tile2 = Tile::default();
    }

    pub(crate) const fn new(id: u8, tile1: Tile, tile2: Tile) -> Domino {
        Domino {
            selectable: false,
            id,
            tile1,
            tile2
        }
    }

    //Draw a domino from the deck pick random number 1-48?
    fn draw() -> Domino {
        Domino::default()
    }

    //Select a domino from the pool
    fn select(id: u8) -> Domino {
        Domino::default()
    }

    //Remove domino from the pool
    fn place(id: u8) {

    }

    fn get_tile_1_type (id: u8) -> String {
        //lookup tile 1 type
        "s".to_string()
    }

}
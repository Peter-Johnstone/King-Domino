#[derive(Default, PartialEq, Debug, Copy, Eq)]
#[derive(Clone)]
pub(crate) enum Types {
    Grass,
    Wheat,
    Forest,
    Swamp,
    Water,
    Mine,

    #[default]
    Null,
}


#[derive(Default, Clone, Debug, Copy, Eq)]
#[derive(PartialEq)]
pub(crate) struct Tile {
    tile_type: Types,
    crowns: u8
}

impl Tile {
    pub(crate) const fn new(tile_type: Types, crowns: u8) -> Tile {
        Tile {
            tile_type,
            crowns,
        }
    }
}
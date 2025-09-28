#[derive(Default, PartialEq, Debug, Copy, Eq)]
#[derive(Clone)]
pub(crate) enum Types {
    Grass,
    Wheat,
    Forest,
    Swamp,
    Water,
    Mine,
    Castle,

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
    pub(crate) const fn new(tile_type: Types, crowns: u8) -> Self {
        Self {
            tile_type,
            crowns,
        }
    }
    pub(crate) fn get_type(&self) -> Types {
        self.tile_type
    }
}

use rand::Rng;
use crate::components::tile::Tile;
use crate::components::tile::Types::{Forest, Grass, Mine, Swamp, Water, Wheat};
use crate::components::domino::Domino;
use crate::components::draft::Draft;

pub(crate) const DECK_SIZE: usize = 48;

pub(crate) struct Deck {
    deck: [Domino; DECK_SIZE],
    len: usize,
}

impl Deck {
    pub(crate) fn initial() -> Self {
        Self {
            deck: DOMINO_SET,
            len: DECK_SIZE,
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Picks a random domino from the remaining list (0 - len)
    pub(crate) fn pick_random(&mut self) -> Domino {

        // Can't pick from an empty deck!
        debug_assert!(!self.is_empty());

        let mut rng = rand::rng();
        let num: usize = rng.random_range(0..self.len);
        self.len -= 1;
        self.deck.swap(num, self.len);

        // We've moved the picked domino to the end
        let picked = self.deck[self.len];

        debug_assert!(!picked.is_null(), "Picked a null domino at idx {num}!");

        // Optional but useful for debugging set the "used" domino to null
        self.deck[num] = Domino::null();

        picked
    }


    /// Picks the new dominoes for the draft from the deck
    pub(crate) fn new_draft(&mut self) -> Draft {

        // make sure we have at least 4 dominoes in the deck
        debug_assert!(self.len >= 4);
        Draft::new([self.pick_random(), self.pick_random(),
                       self.pick_random(), self.pick_random()])
    }

}


pub const DOMINO_SET: [Domino; 48] =
[
    Domino::new(1, Tile::new(Wheat, 0), Tile::new(Wheat, 0)),
    Domino::new(2, Tile::new(Wheat, 0), Tile::new(Wheat, 0)),
    Domino::new(3, Tile::new(Forest, 0), Tile::new(Forest, 0)),
    Domino::new(4, Tile::new(Forest, 0), Tile::new(Forest, 0)),
    Domino::new(5, Tile::new(Forest, 0), Tile::new(Forest, 0)),
    Domino::new(6, Tile::new(Forest, 0), Tile::new(Forest, 0)),
    Domino::new(7, Tile::new(Water, 0), Tile::new(Water, 0)),
    Domino::new(8, Tile::new(Water, 0), Tile::new(Water, 0)),
    Domino::new(9, Tile::new(Water, 0), Tile::new(Water, 0)),
    Domino::new(10, Tile::new(Grass, 0), Tile::new(Grass, 0)),
    Domino::new(11, Tile::new(Grass, 0), Tile::new(Grass, 0)),
    Domino::new(12, Tile::new(Swamp, 0), Tile::new(Swamp, 0)),

    Domino::new(13, Tile::new(Wheat, 0), Tile::new(Forest, 0)),
    Domino::new(14, Tile::new(Wheat, 0), Tile::new(Water, 0)),
    Domino::new(15, Tile::new(Wheat, 0), Tile::new(Grass, 0)),
    Domino::new(16, Tile::new(Wheat, 0), Tile::new(Swamp, 0)),
    Domino::new(17, Tile::new(Forest, 0), Tile::new(Water, 0)),
    Domino::new(18, Tile::new(Forest, 0), Tile::new(Grass, 0)),
    Domino::new(19, Tile::new(Wheat, 1), Tile::new(Forest, 0)),
    Domino::new(20, Tile::new(Wheat, 1), Tile::new(Water, 0)),
    Domino::new(21, Tile::new(Wheat, 1), Tile::new(Grass, 0)),
    Domino::new(22, Tile::new(Wheat, 1), Tile::new(Swamp, 0)),
    Domino::new(23, Tile::new(Wheat, 1), Tile::new(Mine, 0)),
    Domino::new(24, Tile::new(Forest, 1), Tile::new(Wheat, 0)),


    Domino::new(25, Tile::new(Forest, 1), Tile::new(Wheat, 0)),
    Domino::new(26, Tile::new(Forest, 1), Tile::new(Wheat, 0)),
    Domino::new(27, Tile::new(Forest, 1), Tile::new(Wheat, 0)),
    Domino::new(28, Tile::new(Forest, 1), Tile::new(Water, 0)),
    Domino::new(29, Tile::new(Forest, 1), Tile::new(Wheat, 0)),
    Domino::new(30, Tile::new(Water, 1), Tile::new(Wheat, 0)),
    Domino::new(31, Tile::new(Water, 1), Tile::new(Wheat, 0)),
    Domino::new(32, Tile::new(Water, 1), Tile::new(Forest, 0)),
    Domino::new(33, Tile::new(Water, 1), Tile::new(Forest, 0)),
    Domino::new(34, Tile::new(Water, 1), Tile::new(Forest, 0)),
    Domino::new(35, Tile::new(Water, 1), Tile::new(Forest, 0)),
    Domino::new(36, Tile::new(Wheat, 0), Tile::new(Grass, 1)),

    Domino::new(37, Tile::new(Water, 0), Tile::new(Grass, 1)),
    Domino::new(38, Tile::new(Wheat, 0), Tile::new(Swamp, 1)),
    Domino::new(39, Tile::new(Grass, 0), Tile::new(Swamp, 1)),
    Domino::new(40, Tile::new(Mine, 1), Tile::new(Wheat, 0)),
    Domino::new(41, Tile::new(Wheat, 0), Tile::new(Grass, 2)),
    Domino::new(42, Tile::new(Water, 0), Tile::new(Grass, 2)),
    Domino::new(43, Tile::new(Wheat, 0), Tile::new(Swamp, 2)),
    Domino::new(44, Tile::new(Grass, 0), Tile::new(Swamp, 2)),
    Domino::new(45, Tile::new(Mine, 2), Tile::new(Wheat, 0)),
    Domino::new(46, Tile::new(Swamp, 0), Tile::new(Mine, 2)),
    Domino::new(47, Tile::new(Swamp, 0), Tile::new(Mine, 2)),
    Domino::new(48, Tile::new(Wheat, 0), Tile::new(Mine, 3)),
];
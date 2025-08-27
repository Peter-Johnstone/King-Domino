use num_enum::TryFromPrimitive;
use crate::turn::Turn;
use crate::domino::Domino;
use crate::grid::Grid;


#[derive(Clone)]
pub(crate) struct Player {
    my_turn: Turn,
    picked: Domino,
    grid: Grid,
}

impl Player {
    fn new(my_turn: Turn, picked: Domino, grid: Grid) -> Player {
        Player {
            my_turn,
            picked,
            grid,
        }
    }
    
    pub(crate) fn last_picked(&self) -> Domino {
        self.picked.clone()
    }
}
use crate::components::domino::Domino;
use crate::components::grid::Grid;
use crate::components::turn::Turn;


#[derive(Clone, Copy)]
pub(crate) struct Player {
    my_turn: Turn,
    picked: Domino,
    grid: Grid,
}

impl Player {
    pub(crate) fn new(my_turn: Turn) -> Self {
        Self {
            my_turn,
            picked: Domino::null(),
            grid: Grid::empty(),
        }
    }
    

    pub(crate) fn has_placed_all_dominoes(&self) -> bool {
        self.grid.is_full()
    }


    /// Returns the domino that this player picked last
    pub(crate) fn last_picked(&self) -> Domino {
        self.picked
    }
}
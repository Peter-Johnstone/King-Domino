use crate::components::domino::Domino;
use crate::components::grid::Grid;

#[derive(Clone, Copy)]
pub(crate) struct Player {
    id: u8,
    picked: Domino,
    placing: Domino,
    grid: Grid,
}

impl Player {
    pub(crate) fn new(id: u8) -> Self {
        Self {
            id,
            picked: Domino::null(),
            placing: Domino::null(),
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

    /// Ensures that the player has a domino to be placed. Useful during the first round of the game
    pub(crate) fn is_not_placing(&self) -> bool {
        self.placing.is_null()
    }

    pub(crate) fn id(&self) -> u8 {
        self.id
    }

    pub(crate) fn update_last_picked(&mut self, domino: Domino) {
        // We are now placing the domino we stored from last round.
        self.placing = self.picked;

        // Now we cache the new picked domino
        self.picked = domino;


    }

}
use crate::components::domino::Domino;
use crate::components::tile::Tile;
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

    // TODO: IMPLEMENT/
    pub(crate) fn domino_placement(&self) {
        /*

        X) Display message saysing "press r to rotate, 
            click to place domino into a socket. A socket
            represents where the 'anchor tile' will be placed (the one with the hand on it)
            TLDR the tile with the hand on it will be placed into the green square.
        X) generate list of grids

        X) partition list of grids into 4 groups. Each group should contain all valid conformations
            for one orientation
        --GUI--
        X) Draw hand
        X) starting with UP, gui should display all sockets valid for that group, cycling to the next group
            when 'r' is pressed
        X) self.placing = Domino::null()
        X) undraw picked domino
        X) Undraw hand
        */

        // use **FACTS AND LOGIC** to find all possible placements for grid. Save as a vector of grids
        // Also each grid is a vector of vectors... so... Also each grid has Tiles specifically
        // let grid_vec: Vec<Vec<Vec<Tile>>> = self.grid.some_func(self.picked);
        // Have gui allow the user to parse the possible options
        return;
    }

}
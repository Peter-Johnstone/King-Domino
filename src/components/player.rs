use crate::components::domino::Domino;
use crate::components::tile::Tile;
use crate::components::grid::Grid;
use crate::components::deck::Deck;

#[derive(Clone)]
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
            grid: Grid::new(),
        }
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

    pub(crate) fn picked(&self)->Domino{self.picked}

    // TODO: IMPLEMENT/
    pub(crate) fn domino_placement(&mut self) {
        self.placing = self.picked;
        self.picked = Domino::null(); //update state
        let has_room_left = self.grid.build_maps(self.placing); //builds botmaps and the directional bool maps
        if !has_room_left {
            println!("there was no room left for player {}", self.id()); // occurs when the bot_maps vec is of len ==0
            return;
        }
        /*
TODO:

        --GUI--
        X) gui should display all sockets valid for self.domino_rotation
        X) Gui needs to detect a click when each socket is pressed, sends socket and rotation back here.
        X) self.grid.position_selected(&mut self, self.picking.id(), x, y, rotation in radians)
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
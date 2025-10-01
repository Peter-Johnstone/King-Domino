use crate::components::domino::Domino;
use crate::components::grid::Grid;
use crate::components::grid_domino::{self, GridDomino};
use crate::gui::{Gui, PlacementDominoRotation};

#[derive(Clone)]
pub(crate) struct Player {
    id: u8,
    picked: Domino,
    placing: Domino,
    grid: Grid,
    name: String,
}

impl Player {
    pub(crate) fn new(id: u8, name: &str) -> Self {
        Self {
            id,
            picked: Domino::null(),
            placing: Domino::null(),
            grid: Grid::new(),
            name: name.to_string(),
        }
    }
    
    // /// Returns the domino that this player picked last
    // pub(crate) fn last_picked(&self) -> Domino {
    //     self.picked
    // }

    /// Ensures that the player has a domino to be placed. Useful during the first round of the game
    pub(crate) fn is_not_placing(&self) -> bool {
        self.placing.is_null()
    }

    pub(crate) fn id(&self) -> u8 {self.id}

    pub(crate) fn name(&self) -> &String {&self.name}

    pub(crate) fn grid(&self) -> &Grid {&self.grid}

    pub(crate) fn update_last_picked(&mut self, domino: Domino) {
        assert_ne!(domino.id(), 100, "the domino id is {}", domino.id());
        // We are now placing the domino we stored from last round.
        self.placing = domino;

        // Now we cache the new picked domino
        self.picked = Domino::null();


    }

    // pub(crate) fn picked(&self)->Domino{self.picked}
    pub(crate) fn placing(&self)->Domino{self.placing}

    // TODO: IMPLEMENT/
    pub(crate) fn domino_placement(&mut self, curr_orientation: &PlacementDominoRotation, curr_socket_vec: &Vec<[f32; 6]>) -> Option<GridDomino> {
        assert_ne!(self.placing.id(), 100, "For some reason the maps are trying to be built with a null domino");
        let has_room_left = self.grid.build_maps(self.placing); //builds botmaps and the directional bool maps
        if !has_room_left {
            println!("there was no room left for player {}", self.id()); // occurs when the bot_maps vec is of len ==0
            return None;
        }
        let new_domino: Option<GridDomino> = Gui::picked_socket(&self, curr_orientation, curr_socket_vec);
        match new_domino {
            Some(domino) => {
                self.grid.push_domino_map(domino);
                new_domino
            }
            None => {new_domino}
        }
        /*
TODO:

        --GUI--
        X) Gui needs to detect a click when each socket is pressed, sends griddomino back here.
        X) self.grid.position_selected(&mut self, self.picking.id(), x, y, rotation in radians)
        X) self.placing = Domino::null()
        X) undraw picked domino
        X) Undraw hand
        */

        // use **FACTS AND LOGIC** to find all possible placements for grid. Save as a vector of grids
        // Also each grid is a vector of vectors... so... Also each grid has Tiles specifically
        // let grid_vec: Vec<Vec<Vec<Tile>>> = self.grid.some_func(self.picked);
        // Have gui allow the user to parse the possible options
    }

}
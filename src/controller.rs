use macroquad::window::next_frame;
use crate::components::deck::Deck;
use crate::components::domino::Domino;
use crate::components::draft::Draft;
use crate::components::turn::Turn;
use crate::gui::Gui;
use crate::components::player::Player;
use crate::components::turn::Turn::{Prio1};


pub enum Phase {
    Picking,
    Placing
}


pub struct Controller {
    gui: Gui,
    phase: Phase,
    current_turn: Turn,
    pick_draft: Draft,
    place_draft: Draft,
    deck: Deck,
    players: [Player; 4],
    active_player_id: usize,
}

impl Controller {


    /// Create a controller object for the starting game state.
    pub async fn new() -> Self {

        let players = [
            Player::new(1),
            Player::new(2),
            Player::new(3),
            Player::new(4),
        ];

        let mut deck = Deck::initial();
        let draft = deck.new_draft();


        Self {
            gui:            Gui::new().await,
            phase:          Phase::Placing, //The anatomy of a turn is: P1 place, P1 pick, P2 place, P2 pick...
            current_turn: Prio1,
            pick_draft:     draft,
            place_draft:    Draft::null(),
            deck,
            players,
            active_player_id: 0
        }
    }



    /// Starts the game.
    pub async fn start(&mut self) {

        self.run().await;
    }


    /// Main game loop.
    async fn run(&mut self) {

        let running = true;

        while running {

            self.update(); 
            self.gui.draw(&self.pick_draft, &self.place_draft, &self.active_player_id, &self.phase);
            next_frame().await;

        }
    }



    fn update(&mut self) {


        let idx = self.current_turn.idx();

        // Active player is the player at the current priority in the (possibly re-ordered) players array.
        self.active_player_id = self.players[idx].id() as usize;

        match self.phase {


            //will cycle 4 times per turn cycle
            Phase::Placing => {
                //Last turn skipping
                if self.players[idx].has_no_room_left() {
                    print!("GAME OVER for player {}", idx);
                    self.advance_turn();
                    return;
                }
                let temp_player = self.players.get(self.active_player_id-1);
                match temp_player {
                    Some(temp_player) => {
                        temp_player.domino_placement();
                    }
                    None => {
                        eprintln!("Out of bounds error when accessing the players array before domino_placement")
                    }
                }
                
                self.advance_turn();
                self.phase = Phase::Picking;
            }

            Phase::Picking => {
                let picked = {
                    // pass a mutable reference so GUI/pick logic can update player state if needed
                    let player_ref = &mut self.players[idx];
                    Gui::picked_draft_domino(&mut self.pick_draft, player_ref)
                };

                if let Some(domino) = picked {
                    self.players[idx].update_last_picked(domino);

                    //First turn Skipping
                    if self.players[idx].is_not_placing() {
                        // Annoying exception during the first round of the game. We pick but do not place.
                        self.advance_turn();
                        return;
                    } else {
                        self.phase = Phase::Placing;
                    }
                }
            }

        }
    }




    /// Performs the turn of the active player
    ///
    /// 1: Place the domino that we picked in the previous round
    /// 2: Pick a new domino from the available domino list
    /// 3: Advance the turn
    fn perform_turn(&mut self) {

        // Place cached domino
        let last_picked = self.players[self.current_turn.idx()].last_picked();

        // TODO: this is wrong! on the first turn of the game, the last_picked SHOULD be null
        debug_assert_ne!(last_picked, Domino::null());


        // Pick from draft
        // TODO: we need to implement the logic for actually deciding which domino to pick
        let pick_idx = 0;


        self.advance_turn();
    }


    /// Advanced the turn, performing any necessary actions
    /// to do so (or ending the game, if over)
    fn advance_turn(&mut self) {

        if self.game_over() {
            return;
        }

        if self.pick_draft.is_empty() {
            // All players have picked from the current pick_draft. Turn it into the place_draft
            // and start the placing phase. Reset the current turn to the first placement priority.
            self.place_draft = self.pick_draft.clone();
            self.pick_draft = self.deck.new_draft();
            self.place_draft.apply_new_order(&mut self.players);

            // Restart Draft with first Priority
            self.current_turn = Prio1;
            return;
        }

        // Otherwise simply advance to the next player for picking
        self.current_turn.advance();
    }



    /// Ends the game
    fn end_game(&mut self) {
        return
    }

    /// Returns true iff all players have placed their last tile
    fn game_over(&self) -> bool {
        false
    }


}


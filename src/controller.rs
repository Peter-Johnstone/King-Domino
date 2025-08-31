use macroquad::window::next_frame;
use crate::components::deck::Deck;
use crate::components::domino::Domino;
use crate::components::draft::Draft;
use crate::components::turn::Turn;
use crate::gui::Gui;
use crate::components::player::Player;
use crate::components::turn::Turn::{Player1, Player2, Player3, Player4};


enum Phase {
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
}

impl Controller {


    /// Create a controller object for the starting game state.
    pub async fn new() -> Self {

        let players = [
            Player::new(Player1),
            Player::new(Player2),
            Player::new(Player3),
            Player::new(Player4),
        ];

        let mut deck = Deck::initial();
        let draft = deck.new_draft();


        Self {
            gui:            Gui::new().await,
            phase:          Phase::Picking,
            current_turn:   Player1,
            pick_draft:     draft,
            place_draft:    Draft::null(),
            deck,
            players
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
            self.gui.draw(&self.pick_draft, &self.place_draft);
            next_frame().await;

        }
    }



    fn update(&mut self) {

        let idx = self.current_turn.idx();
        match self.phase {

            Phase::Picking => {
                let picked = {
                    let player_ref = &self.players[idx];
                    Gui::picked_draft_domino(&mut self.pick_draft, player_ref)
                };

                if let Some(domino) = picked {
                    self.players[idx].update_last_picked(domino);
                    self.phase = Phase::Placing;
                }
            }

            Phase::Placing => {
                if self.players[idx].is_not_placing() {
                    // Annoying exception during the first round of the game. We pick but do not place.
                    self.phase = Phase::Picking;
                    self.advance_turn();
                    return;
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
            // we need to deal the new draft
            println!("Getting here!");
            self.place_draft = self.pick_draft.clone();
            self.pick_draft = self.deck.new_draft();
        }


        // Switch the active player
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


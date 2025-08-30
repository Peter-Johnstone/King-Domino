use macroquad::input::{is_key_pressed, is_mouse_button_pressed, MouseButton};
use macroquad::prelude::KeyCode;
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


pub(crate) struct Controller {
    gui: Gui,
    phase: Phase,
    active_player: Turn,
    draft: Draft,
    deck: Deck,
    players: [Player; 4],
}

impl Controller {


    /// Create a controller object for the starting game state.
    pub(crate) async fn new() -> Self {

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
            active_player:  Player1,
            draft,
            deck,
            players
        }
    }



    /// Starts the game.
    pub(crate) async fn start(&mut self) {

        self.run().await;
    }


    /// Main game loop.
    async fn run(&mut self) {

        let running = true;

        while running {

            self.update();
            self.gui.draw(&self.draft);
            next_frame().await;

        }
    }






    fn update(&mut self) {

        match self.phase {

            Phase::Picking => {

            }

            Phase::Placing => {

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
        let last_picked = self.players[self.active_player as usize].last_picked();

        // TODO: this is wrong! on the first turn of the game, the last_picked SHOULD be null
        debug_assert_ne!(last_picked, Domino::null());


        // Pick from draft
        // TODO: we need to implement the logic for actually deciding which domino to pick
        let pick_idx = 0;
        self.draft.pick(0);


        self.advance_turn();
    }


    /// Advanced the turn, performing any necessary actions
    /// to do so (or ending the game, if over)
    fn advance_turn(&mut self) {

        if self.game_over() {
            return;
        }

        if self.draft.is_empty() {
            // we need to deal the new draft
            self.draft = self.deck.new_draft();
        }


        // Switch the active player
        self.active_player.advance();
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


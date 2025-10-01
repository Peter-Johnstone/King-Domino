use macroquad::window::next_frame;
use crate::components::deck::Deck;
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
    turn_number:u8,
    subturn_number: u8, // This is from 1-4, helps the gui know how many dominos to remove from the drawn draft
}

impl Controller {


    /// Create a controller object for the starting game state.
    pub async fn new() -> Self {

        let players = [
            Player::new(1, "Blue"),
            Player::new(2, "Green"),
            Player::new(3, "Red"),
            Player::new(4, "Yellow"),
        ];

        let mut deck = Deck::initial();
        let draft = deck.new_draft();


        Self {
            gui:            Gui::new().await,
            phase:          Phase::Picking, // the game will crash if you make this start as Placing, since it tries to draw the default domino
            current_turn: Prio1,
            pick_draft:     draft,
            place_draft:    Draft::null(),
            deck,
            players,
            active_player_id: 0,
            turn_number: 0,
            subturn_number: 0 // technically starts at zero since we dont remove the domino on the first turn. used to trim draft gui
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
            self.gui.draw(&self.pick_draft, &self.place_draft, &self.active_player_id, &self.phase, &self.players, &self.subturn_number);
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
                self.gui.check_r_key_pressed();
                let temp_player = self.players.get_mut((self.subturn_number-1) as usize);
                let mut ready_to_place: bool = true;
                match temp_player {
                    Some(temp_player) => {
                        ready_to_place = temp_player.domino_placement();
                    }
                    None => {
                        eprintln!("Out of bounds error when accessing the players array before domino_placement")
                    }
                }
                
                // self.advance_turn();
                if ready_to_place {self.phase = Phase::Picking;}
            }

            Phase::Picking => {
                
                let picked = {
                    // pass a mutable reference so GUI/pick logic can update player state if needed
                    let player_ref = &mut self.players[idx];
                    Gui::picked_draft_domino(&mut self.pick_draft, player_ref)
                };

                if let Some(domino) = picked {
                    self.subturn_number = self.subturn_number.wrapping_add(1);
                    self.players[idx].update_last_picked(domino);
                    println!("Player {} has picked domino of id {}", self.players[idx].name(), self.players[idx].placing().id());

                    
                    if self.turn_number == 0 && self.subturn_number != 4{ //First turn the placing phase is skipped
                        self.advance_turn();
                        return;
                    }

                    if self.pick_draft.is_empty() {
                        self.advance_turn();
                    }

                    if self.subturn_number%4==0{self.subturn_number=1; self.turn_number+=1;}
                    self.phase = Phase::Placing;
                }
            }

        }
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



    // /// Ends the game
    // fn end_game(&mut self) {
    //     return
    // }

    /// Returns true iff all players have placed their last tile
    fn game_over(&self) -> bool {
        false
    }


}


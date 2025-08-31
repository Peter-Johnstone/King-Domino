use std::collections::HashMap;
use macroquad::miniquad::FilterMode;
use macroquad::prelude::{load_texture, Texture2D};
use crate::components::deck::DECK_SIZE;
use crate::components::turn::NUM_PLAYERS;

pub(crate) struct Assets {

    // dictionary with u8 key between 0-47 representing the
    domino_by_id:   HashMap<u8, Texture2D>,
    king_by_turn: HashMap<u8, Texture2D>,
}


impl Assets {



    /// Loads all the assets. Ideally, this should only ever be called once.
    pub(crate) async fn load() -> Self {

        let king_by_turn = Self::load_king_textures().await;
        let domino_by_id = Self::load_domino_textures().await;

        Self {
            king_by_turn,
            domino_by_id
        }
    }

    async fn load_domino_textures() -> HashMap<u8, Texture2D> {
        let mut domino_by_id = HashMap::new();

        // Creates the dictionary to be able to fetch the domino texture from the domino id
        for id in 1..DECK_SIZE + 1 {


            // TODO: fix this path
            let path = format!("res/img/dominoes/domino_{}.png", id % 20 + 1); // +1 because our img names start with 1

            // TODO: Add other domino images.
            //let path = format!("../res/img/domino_{}.png", id); // +1 because our img names start with 1

            let texture = load_texture(&path).await.unwrap();
            texture.set_filter(FilterMode::Nearest);

            domino_by_id.insert(id as u8, texture);
        }
        domino_by_id
    }

    async fn load_king_textures() -> HashMap<u8, Texture2D> {
        let mut player_by_turn = HashMap::new();
        for turn in 1..NUM_PLAYERS + 1 {
            let path = format!("res/img/kings/king_{}.png", turn);

            let texture = load_texture(&path).await.unwrap();
            texture.set_filter(FilterMode::Nearest);

            player_by_turn.insert(turn, texture);
        }
        player_by_turn
    }

    /// Fetches the king texture given a player's id (turn) between 1 and 4
    pub(crate) fn fetch_king_texture_by_turn(&self, id: u8) -> Option<&Texture2D> {

        // We should never fetch an impossible id
        assert!(self.king_by_turn.contains_key(&(id+1)), "id: {id}");

        self.king_by_turn.get(&(id+1))
    }

    /// Fetches the domino texture given an id between 1 and 48
    pub(crate) fn fetch_domino_texture_by_id(&self, id: u8) -> Option<&Texture2D> {

        // We should never fetch an impossible id
        assert!(self.domino_by_id.contains_key(&id), "id: {id}");

        self.domino_by_id.get(&id)
    }


}
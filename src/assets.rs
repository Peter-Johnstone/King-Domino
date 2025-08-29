use std::collections::HashMap;
use macroquad::prelude::Texture2D;
use crate::components::deck::DECK_SIZE;

struct Assets {
    domino_by_id: HashMap<u8, Texture2D>,
}


impl Assets {


    pub(crate) async fn load() {

        for id in 0..DECK_SIZE {


            let path = format!("assets/{}.png", id);


        }



    }


}
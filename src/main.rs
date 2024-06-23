/*
Implementation of Glicko2 Rating System
Paper: http://www.glicko.net/glicko/glicko2.pdf
*/

mod battle;
mod structs;

use battle::{battles, pick_2_player_ids};

use crate::structs::{initialize_characters, store_characters};

fn main() {
    let mut characters = initialize_characters();
    let records = battles(&mut characters);
    store_characters(&characters);
}

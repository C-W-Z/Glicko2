/*
Implementation of Glicko2 Rating System
Paper: http://www.glicko.net/glicko/glicko2.pdf
*/

mod structs;
mod battle;

use battle::pick_2_player_ids;

use crate::structs::{initialize_characters, store_characters};

fn main() {
    let characters = initialize_characters();
    let res = pick_2_player_ids(&characters);
    println!("{} {}", res[0], res[1]);
    store_characters(&characters);
}

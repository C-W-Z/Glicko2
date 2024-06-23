/*
Implementation of Glicko2 Rating System
Paper: http://www.glicko.net/glicko/glicko2.pdf
*/

mod structs;

use crate::structs::{initialize_characters, store_characters};

fn main() {
    let characters = initialize_characters();
    store_characters(characters);
}

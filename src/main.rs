/*
Implementation of Glicko2 Rating System
Paper: http://www.glicko.net/glicko/glicko2.pdf
*/

mod battle;
mod glicko;
mod structs;

use crate::battle::battles;
use crate::glicko::calculate_results;
use crate::structs::{initialize_characters, store_characters};

fn main() {
    let mut characters = initialize_characters();
    let records = battles(&mut characters);
    calculate_results(&mut characters, records);
    store_characters(&characters);
}

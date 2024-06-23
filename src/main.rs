/*
Implementation of Glicko2 Rating System
Paper: http://www.glicko.net/glicko/glicko2.pdf
*/

mod battle;
mod display;
mod glicko;
mod structs;

use crate::battle::battles;
use crate::display::list_ranking;
use crate::glicko::{calculate_ranking, calculate_results, update_history};
use crate::structs::{initialize_characters, store_characters};

fn main() {
    let mut characters = initialize_characters();
    let (mut ranked_chara, mut ranks) = calculate_ranking(&characters);
    let records = battles(&characters);
    update_history(&mut characters, &records, &ranks);
    calculate_results(&mut characters, &records);
    (ranked_chara, ranks) = calculate_ranking(&characters);
    list_ranking(&ranked_chara, &ranks);
    store_characters(&characters);
}

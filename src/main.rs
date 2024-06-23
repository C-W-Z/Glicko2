/*
Implementation of Glicko2 Rating System
Paper: http://www.glicko.net/glicko/glicko2.pdf
*/

mod battle;
mod glicko;
mod structs;

use glicko::update_history;

use crate::battle::battles;
use crate::glicko::{calculate_results, calculate_ranking};
use crate::structs::{initialize_characters, store_characters};

fn main() {
    let mut characters = initialize_characters();
    let (mut ranked_chara, mut ranks) = calculate_ranking(&characters);
    let records = battles(&characters);
    update_history(&mut characters, &records, &ranks);
    calculate_results(&mut characters, &records);
    (ranked_chara, ranks) = calculate_ranking(&characters);
    for c in ranked_chara.iter() {
        println!("#{}: {} {} ± {}", ranks[&c.id], c.name, c.rank.rati, c.rank.devi);
    }
    store_characters(&characters);
}

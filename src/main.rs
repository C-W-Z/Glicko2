/*
Implementation of Glicko2 Rating System
Paper: http://www.glicko.net/glicko/glicko2.pdf
*/

mod battle;
mod display;
mod glicko;
mod structs;

use std::io::{self, Write};
use crate::battle::battles;
use crate::display::{list_ranking, stat};
use crate::glicko::{calculate_ranking, calculate_results, update_history};
use crate::structs::{initialize_characters, store_characters};

fn main() {
    let mut characters = initialize_characters();
    let (mut ranked_chara, mut ranks) = calculate_ranking(&characters);
    
    let mut choice: String = String::new();
    println!("=========~ Glicko2: Lobby ~=========");
    display::lobby_help();
    loop {
        print!("Lobby >> ");
        let _ = io::stdout().flush();
        choice.clear();

        let _ = io::stdin().read_line(&mut choice);

        choice = choice.trim().to_string();
        if choice.starts_with("star") {
            let records = battles(&characters);
            update_history(&mut characters, &records, &ranks);
            calculate_results(&mut characters, &records);
            (ranked_chara, ranks) = calculate_ranking(&characters);
        } else if choice.starts_with("list") {
            list_ranking(&ranked_chara, &ranks);
        } else if choice.starts_with("stat") {
            match choice.split_off(4).trim().parse::<usize>() {
                Ok(id) => stat(&characters[id], &ranked_chara, &ranks),
                Err(_) => {
                    display::lobby_stat_help();
                    continue;
                }
            }
        } else if choice.starts_with("help") {
            display::lobby_help();
        } else {
            break;
        }
    }

    store_characters(&characters);
}

/*
Implementation of Glicko2 Rating System
Paper: http://www.glicko.net/glicko/glicko2.pdf
*/

mod battle;
mod display;
mod glicko;
mod structs;

use crate::battle::battles;
use crate::display::{list_ranking, stat};
use crate::glicko::{calculate_ranking, calculate_results, update_history};
use crate::structs::{initialize_characters, store_characters, Character};
use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let (mut characters, name_to_id) = initialize_characters();
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
            let records = battles(&characters, &name_to_id);
            update_history(&mut characters, &records, &ranks);
            calculate_results(&mut characters, &records);
            (ranked_chara, ranks) = calculate_ranking(&characters);
        } else if choice.starts_with("l") {
            list_ranking(&ranked_chara, &ranks);
        } else if choice.starts_with("stat") {
            handle_stat(&mut choice, &characters, &name_to_id, &ranked_chara, &ranks);
        } else if choice.starts_with("h") {
            display::lobby_help();
        } else {
            break;
        }
    }

    store_characters(&characters);
}

fn handle_stat(
    choice: &mut String,
    characters: &[Character],
    name_to_id: &HashMap<String, usize>,
    ranked_chara: &[Character],
    ranks: &HashMap<usize, usize>,
) {
    let mut ch = choice.split_off(4);
    ch = ch.trim().to_string();

    match ch.parse::<usize>() {
        Ok(id) => {
            stat(&characters[id], characters, name_to_id, ranked_chara, ranks);
            return;
        }
        Err(_) => {}
    }

    match name_to_id.get(&ch) {
        Some(id) => stat(
            &characters[*id],
            &characters,
            &name_to_id,
            &ranked_chara,
            &ranks,
        ),
        None => {
            display::lobby_stat_help();
        }
    };
}

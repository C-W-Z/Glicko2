use crate::structs::Character;
use std::collections::HashMap;

pub fn list_ranking(ranked_chara: &[Character], ranks: &HashMap<usize, usize>) {
    for c in ranked_chara.iter() {
        let entry = format!("{:<4} {:<26}{}",
            format!("{}.", ranks[&c.id]),
            c.name,
            format!("({0: <7} ± {1:.0})",
                format!("{:.2}", c.rank.rati),
                c.rank.devi
            )
        );
        println!("{}", entry);
    }
}

pub fn start_fight(battle_id: usize, left: &str, right: &str) {
    println!("-----------------------------");
    println!("Battle #{}: {} vs {}", battle_id + 1, left, right);
    print!("Pick [ 'h' for help ] >> ");
}

pub fn fight_result(result: &str) {
    println!("Chose - {}!", result);
}

pub fn fight_dislike_both() {
    println!("Disliked both!");
}

pub fn fight_undo() {
    println!("Undoing...");
}

pub fn fight_undo_err() {
    println!("This is the first battle!");
}

pub fn fight_help() {
    println!("1/2 to choose left/right");
    println!("0 for draws");
    println!("d if you DISLIKE BOTH of them");
    println!("u to UNDO");
    println!("<Enter> to end this session");
}

pub fn fight_end() {
    println!("Finish rating session.");
}

pub fn start_session(num_characters: usize) {
    println!(
        "=== Starting a new session with {} characters ===",
        num_characters
    );
}
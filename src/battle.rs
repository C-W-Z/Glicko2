use rand::distributions::WeightedIndex;
use rand::prelude::*;
use std::io::{self, Write};

use crate::structs::{BattleStat, Character, Match, MatchResult};

pub fn pick_2_player_ids(pool: &[Character]) -> (usize, usize) {
    let max = pool
        .iter()
        .max_by_key(|c| c.hist.battles())
        .unwrap()
        .hist
        .battles();

    // Calculate the weights (inverse of the battles)
    let weights: Vec<_> = pool
        .iter()
        // Plus one is to give the character with the most battles a small chance of being selected
        .map(|c: &Character| max - c.hist.battles() + 1)
        .collect();

    // Create a weighted index distribution
    let distribution = WeightedIndex::new(&weights).unwrap();
    let mut rng = thread_rng();

    // Select two distinct indices
    let first_index = distribution.sample(&mut rng);
    let mut second_index = distribution.sample(&mut rng);
    while second_index == first_index {
        second_index = distribution.sample(&mut rng);
    }

    (first_index, second_index)
}

pub fn fight(battle_id: usize, left: &str, right: &str) -> (MatchResult, BattleStat) {
    let mut choice: String = Default::default();
    loop {
        println!("-----------------------------");
        println!("Battle #{}: {} vs {}", battle_id, left, right);
        print!("Pick [ 'h' for help ] >> ");
        let _ = io::stdout().flush();
        choice.clear();

        let _ = io::stdin().read_line(&mut choice);

        let mut res = MatchResult::Draw;

        choice = choice.trim().to_string();
        if choice.ends_with('1') {
            // I like left
            res = MatchResult::AWin;
            println!("Chose - {}!", left);
        } else if choice.ends_with('2') {
            // I like right
            res = MatchResult::BWin;
            println!("Chose - {}!", right);
        } else if choice.ends_with("0") {
            // Draw
            println!("Chose - Draw!");
            res = MatchResult::Draw;
        } else if choice.ends_with('d') {
            // I dislike them both!
            res = MatchResult::BothLose;
            println!("Disliked both!");
        } else if choice.ends_with('u') {
            // Undo
            if battle_id == 0 {
                println!("This is the first battle!");
                continue;
            }
            println!("Going back...");
            return (res, BattleStat::Undo);
        } else if choice.ends_with('h') {
            // Help
            println!("1/2 to choose left/right");
            println!("0 for draws");
            println!("d if you DISLIKE BOTH of them");
            println!("u to UNDO");
            println!("<Enter> to end this session");
            continue;
        } else {
            // End
            println!("Finishing rating period...");
            return (res, BattleStat::End);
        }

        return (res, BattleStat::Next);
    }
}

pub fn battles(pool: &mut Vec<Character>) -> Vec<Match> {
    println!(
        "=== Starting a new session with {} characters ===",
        pool.len()
    );

    let mut records: Vec<Match> = Vec::new();

    let (mut left, mut right) = pick_2_player_ids(&pool);

    loop {
        let (res, stat) = fight(0, pool[left].name.as_str(), pool[right].name.as_str());

        match stat {
            BattleStat::Next => {
                records.push(Match {
                    a: (left),
                    b: (right),
                    res: (res),
                });
                (left, right) = pick_2_player_ids(&pool);
            }
            BattleStat::End => {
                records.push(Match {
                    a: (left),
                    b: (right),
                    res: (res),
                });
                break;
            }
            BattleStat::Undo => {
                let last = records.last().unwrap();
                left = last.a;
                right = last.b;
                records.pop();
            }
        }
    }

    records
}

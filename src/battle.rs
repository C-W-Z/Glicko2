use crate::{
    display,
    structs::{BattleStat, Character, Match, MatchResult},
};
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    io::{self, Write}
};

fn pick_2_player_ids(pool: &[Character], name_to_id: &HashMap<String, usize>) -> (usize, usize) {
    let max = pool
        .iter()
        .max_by_key(|c| c.hist.battles())
        .unwrap()
        .hist
        .battles();

    // Calculate the weights (inverse of the battles)
    let weights: Vec<_> = pool
        .iter()
        .map(|c: &Character| (2.0 * (max - c.hist.battles()) as f64).exp2())
        .collect();

    // Create a weighted index distribution
    let distribution = WeightedIndex::new(&weights).unwrap();
    let mut rng = thread_rng();

    // Select two distinct indices
    let first_index = distribution.sample(&mut rng);

    // find recent opponents of first selected
    let mut oppos: HashSet<usize> = HashSet::new();
    for b in pool[first_index].hist.recent.iter() {
        oppos.insert(name_to_id[&b.oppo]);
    }

    let mut second_index = distribution.sample(&mut rng);
    while second_index == first_index || oppos.contains(&second_index) {
        second_index = distribution.sample(&mut rng);
    }

    (first_index, second_index)
}

fn fight(battle_id: usize, left: &str, right: &str) -> (MatchResult, BattleStat) {
    let mut choice: String = String::new();
    loop {
        display::start_fight(battle_id, left, right);
        let _ = io::stdout().flush();
        choice.clear();

        let _ = io::stdin().read_line(&mut choice);

        let mut res = MatchResult::Draw;

        choice = choice.trim().to_string();
        if choice.starts_with('1') {
            // I like left
            res = MatchResult::AWin;
            display::fight_result(left);
        } else if choice.starts_with('2') {
            // I like right
            res = MatchResult::BWin;
            display::fight_result(right);
        } else if choice.starts_with("0") {
            // Draw
            display::fight_result("Draw!");
            res = MatchResult::Draw;
        } else if choice.starts_with('d') {
            // I dislike them both!
            res = MatchResult::BothLose;
            display::fight_dislike_both();
        } else if choice.starts_with('u') {
            // Undo
            if battle_id == 0 {
                display::fight_undo_err();
                continue;
            }
            display::fight_undo();
            return (res, BattleStat::Undo);
        } else if choice.starts_with('h') {
            // Help
            display::fight_help();
            continue;
        } else {
            // End
            display::fight_end();
            return (res, BattleStat::End);
        }

        return (res, BattleStat::Next);
    }
}

pub fn battles(pool: &[Character], name_to_id: &HashMap<String, usize>) -> Vec<Match> {
    display::start_session(pool.len());

    let mut records: Vec<Match> = Vec::new();

    let (mut left, mut right) = pick_2_player_ids(&pool, name_to_id);

    loop {
        let (res, stat) = fight(
            records.len(),
            pool[left].name.as_str(),
            pool[right].name.as_str(),
        );

        match stat {
            BattleStat::Next => {
                records.push(Match::new(left, right, res));
                (left, right) = pick_2_player_ids(&pool, name_to_id);
            }
            BattleStat::End => {
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

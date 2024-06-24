use crate::{
    display,
    structs::{Battle, BattleStat, Character, Match, MatchResult},
};
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    io::{self, Write},
};

const MAX_HIST: usize = 5;

fn update_tmp_history(characters: &mut Vec<Character>, a: usize, b: usize) {
    // characters[m.a].hist.battles() += 1
    characters[a].hist.draw += 1;
    characters[b].hist.draw += 1;

    let name_a = characters[a].name.clone();
    let name_b = characters[b].name.clone();
    characters[a]
        .hist
        .recent
        .push_back(Battle::new(name_b, MatchResult::Draw));
    characters[b]
        .hist
        .recent
        .push_back(Battle::new(name_a, MatchResult::Draw));
    while characters[a].hist.recent.len() > MAX_HIST {
        characters[a].hist.recent.pop_front();
    }
    while characters[b].hist.recent.len() > MAX_HIST {
        characters[b].hist.recent.pop_front();
    }
}

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
        .map(|c: &Character| {
            if max == c.hist.battles() {
                return 0.0;
            } else {
                return (1 << (2 * (max - c.hist.battles()))) as f64;
            }
        })
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

    let mut _pool = pool.to_vec();
    let mut records: Vec<Match> = Vec::new();

    let (mut left, mut right) = pick_2_player_ids(&_pool, name_to_id);

    loop {
        let (res, stat) = fight(
            records.len(),
            pool[left].name.as_str(),
            pool[right].name.as_str(),
        );

        match stat {
            BattleStat::Next => {
                records.push(Match::new(left, right, res));
                update_tmp_history(&mut _pool, left, right);
                (left, right) = pick_2_player_ids(&_pool, name_to_id);
            }
            BattleStat::End => {
                break;
            }
            BattleStat::Undo => {
                let last = records.last().unwrap();
                left = last.a;
                right = last.b;
                records.pop();
                _pool[left].hist.recent.pop_back();
                _pool[right].hist.recent.pop_back();
            }
        }
    }

    records
}

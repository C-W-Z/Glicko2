use crate::structs::{Character, MatchResult};
use std::collections::HashMap;

fn print_rank_entry(c: &Character, rank: usize, tab: usize) {
    print!("{:<1$}", "", tab);
    println!(
        "{:<4} {:<26}{}",
        format!("{}.", rank),
        c.name,
        format!(
            "({0: <7} ± {1:.0})",
            format!("{:.2}", c.rank.rati),
            c.rank.devi
        )
    );
}

fn get_slice_in_ranked_chara<'a>(
    character: &'a Character,
    ranked_chara: &'a [Character],
) -> Vec<&'a Character> {
    for (i, c) in ranked_chara.iter().enumerate() {
        if c.id != character.id {
            continue;
        }
        if i == 0 {
            // top
            return vec![&ranked_chara[0], &ranked_chara[1], &ranked_chara[2]];
        } else if i == ranked_chara.len() - 1 {
            // bottom
            return vec![&ranked_chara[i - 2], &ranked_chara[i - 1], &ranked_chara[i]];
        } else {
            // middle
            return vec![&ranked_chara[i - 1], &ranked_chara[i], &ranked_chara[i + 1]];
        }
    }
    vec![]
}

pub fn stat(
    chara: &Character,
    characters: &[Character],
    ranked_chara: &[Character],
    ranks: &HashMap<usize, usize>,
) {
    println!("{:-<1$}", "", 58);
    println!(
        "{0: <45}{1: >13}",
        format!("~~ {} ~~", chara.name),
        format!("Rank #{}/{}", ranks[&chara.id], ranked_chara.len())
    );
    println!("{:-<1$}", "", 58);

    println!("==> {}", "RATING");
    println!(
        "{}",
        format!(
            "    {} ± {1:.0} | (volatility: {2:.6})",
            format!("{:.2}", chara.rank.rati),
            chara.rank.devi,
            chara.rank.vola
        )
    );
    if chara.rank.devi > 160.0 {
        println!("    ⓘ The uncertainty is high, do more battles!\n");
    }

    if !chara.hist.old_rank.is_empty() {
        println!(
            "    -- Last {} {} --",
            chara.hist.old_rank.len(),
            if chara.hist.old_rank.len() > 1 {
                "sessions"
            } else {
                "session"
            }
        );
        let pt_diff = chara.rank.rati - *chara.hist.old_rate.front().unwrap();
        let rk_diff: isize =
            ranks[&chara.id] as isize - *chara.hist.old_rank.front().unwrap() as isize;
        println!(
            "    {} {:.0} {} {}.",
            if pt_diff > 0.0 {
                "🡽"
            } else if pt_diff == 0.0 {
                "🡺"
            } else {
                "🡾"
            },
            pt_diff.abs(),
            if pt_diff == 1.0 { "point" } else { "points" },
            if pt_diff >= 0.0 { "gained" } else { "lost" }
        );
        println!(
            "    {} {} {} {}.",
            if rk_diff < 0 {
                "🡽"
            } else if rk_diff == 0 {
                "🡺"
            } else {
                "🡾"
            },
            rk_diff.abs(),
            if rk_diff.abs() == 1 {
                "place"
            } else {
                "places"
            },
            if rk_diff <= 0 { "gained" } else { "lost" }
        );
    }

    // Rank informations
    println!("\n==> {}", "RANKINGS");
    // Overall ranks
    let slice = get_slice_in_ranked_chara(chara, ranked_chara);
    println!(
        "\n  - {:<42}{:>8}",
        "Overall",
        format!("#{}/{}", ranks[&chara.id], ranked_chara.len())
    );
    println!("    {:-<50}", "");
    for c in slice.iter() {
        print_rank_entry(c, ranks[&c.id], 4);
    }

    // Stats
    println!("\n==> {}", "STATISTICS");
    let total = chara.hist.battles();
    println!(
        "    Wins:   {} ({}%)",
        chara.hist.wins,
        if total == 0 {
            0
        } else {
            100 * chara.hist.wins / total
        }
    );
    println!("    Draws:  {}", chara.hist.draw);
    println!("    Losses: {}", chara.hist.loss);

    // Recent battles
    if !chara.hist.recent.is_empty() {
        println!("\n==> {}", "RECENT BATTLES");
    }
    for m in chara.hist.recent.iter() {
        let msg = match m.res {
            MatchResult::Draw => "Drew",
            MatchResult::BothLose => "Drew (lost)",
            MatchResult::AWin => {
                if m.a == chara.id {
                    "Won"
                } else {
                    "Lost"
                }
            }
            MatchResult::BWin => {
                if m.b == chara.id {
                    "Won"
                } else {
                    "Lost"
                }
            }
        };
        let oppo = if m.a == chara.id { m.b } else { m.a };
        println!(
            "    {} against {} ({:.0})",
            msg, characters[oppo].name, characters[oppo].rank.rati
        );
    }

    println!();
}

pub fn list_ranking(ranked_chara: &[Character], ranks: &HashMap<usize, usize>) {
    println!("{:-<1$}", "", 58);
    println!("#    Name                      Rating           Extra ");
    println!("{:-<1$}", "", 58);
    for c in ranked_chara.iter() {
        print_rank_entry(c, ranks[&c.id], 0);
    }
}

pub fn lobby_help() {
    println!("-- 'start':   start a new session.");
    println!("-- 'list':    show the ranking list.");
    println!("-- 'stat':    see stats of a character.");
    println!("-------------------------------------");
    println!("-- 'help':    display this message.");
    println!("-- 'exit':    See you next time.");
}

pub fn lobby_stat_help() {
    println!("usage: stat [character ID]");
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

use crate::structs::{Battle, Character, Match, MatchResult};
use std::collections::HashMap;
use std::f64::consts::PI;

// The system constant which constrains the change in volatility over time, needs to be set prior to application of the system
// Reasonable choices are between 0.3 and 1.2
const TAU: f64 = 0.5;
// Convergence tolerance
const EPSILON: f64 = 1e-6;

// The maximum number of old ratings/ranks stored
const MAX_HIST: usize = 5;

fn g(phi: f64) -> f64 {
    1.0 / (1.0 + 3.0 * (phi / PI).powi(2)).sqrt()
}

fn e(mu: f64, mu_j: f64, phi_j: f64) -> f64 {
    1.0 / (1.0 + (-g(phi_j) * (mu - mu_j)).exp())
}

fn part_v(mu: f64, mu_j: f64, phi_j: f64) -> f64 {
    let _e = e(mu, mu_j, phi_j);
    g(phi_j).powi(2) * _e * (1.0 - _e)
}

fn part_d(mu: f64, mu_j: f64, phi_j: f64, s: f64) -> f64 {
    g(phi_j) * (s as f64 - e(mu, mu_j, phi_j))
}

fn new_volatility(v: f64, delta: f64, sigma: f64, phi: f64, tau: f64, epsilon: f64) -> f64 {
    let _a = sigma.powi(2).ln();
    let delta2 = delta * delta;
    let tau2 = tau * tau;
    let phi2plusv = phi * phi + v;

    let f = |x: f64| -> f64 {
        let ex = x.exp();
        (ex * (delta2 - phi2plusv - ex) / (2.0 * (phi2plusv + ex).powi(2))) - ((x - _a) / tau2)
    };

    let mut a = _a;
    let mut b = if delta2 > phi2plusv {
        (delta2 - phi2plusv).ln()
    } else {
        let mut k = 1.0;
        while f(_a - k * tau) < 0.0 {
            k += 1.0;
        }
        _a - (k * tau)
    };

    let mut fa = f(a);
    let mut fb = f(b);
    while (b - a).abs() > epsilon {
        let c = a + (a - b) * fa / (fb - fa);
        let fc = f(c);
        if fc * fb <= 0.0 {
            a = b;
            fa = fb;
        } else {
            fa /= 2.0;
        }
        b = c;
        fb = fc;
    }

    (a / 2.0).exp()
}

fn new_deviation(phi: f64, new_sigma: f64, v: f64) -> f64 {
    let phi_star = (phi * phi + new_sigma * new_sigma).sqrt();
    1.0 / (1.0 / phi_star.powi(2) + 1.0 / v.powi(2)).sqrt()
}

fn new_rating(mu: f64, new_phi: f64, v: f64, delta: f64) -> f64 {
    mu + new_phi * delta / v
}

pub fn calculate_results(characters: &mut Vec<Character>, records: &[Match]) {
    if records.is_empty() {
        return;
    }

    // For each player, convert the ratings and RD’s onto the Glicko-2 scale
    for c in characters.iter_mut() {
        c.rank.glicko_1_to_2_scale();
    }

    // Compute the quantity v
    // This is the estimated variance of the player’s rating based only on game outcomes
    let mut v: HashMap<usize, f64> = HashMap::new();
    // Compute the quantity delta
    // This is the estimated improvement in rating by comparing the pre-period rating to the performance rating based only on game outcomes
    let mut delta: HashMap<usize, f64> = HashMap::new();

    // initialize hashmaps
    for m in records.iter() {
        v.insert(m.a, 0.0);
        v.insert(m.b, 0.0);
        delta.insert(m.a, 0.0);
        delta.insert(m.b, 0.0);
    }

    for m in records.iter() {
        let mu1 = characters[m.a].rank.rati;
        let mu2 = characters[m.b].rank.rati;
        let phi1 = characters[m.a].rank.devi;
        let phi2 = characters[m.b].rank.devi;
        let (s1, s2) = match m.res {
            MatchResult::AWin => (1.0, 0.0),
            MatchResult::BWin => (0.0, 1.0),
            MatchResult::Draw => (0.5, 0.5),
            MatchResult::BothLose => (0.0, 0.0),
        };

        // Add up the quantities calculated by matches with others players
        if let Some(v1) = v.get_mut(&m.a) {
            *v1 += part_v(mu1, mu2, phi2);
        }
        if let Some(v2) = v.get_mut(&m.b) {
            *v2 += part_v(mu2, mu1, phi1);
        }
        if let Some(d1) = delta.get_mut(&m.a) {
            *d1 += part_d(mu1, mu2, phi2, s1);
        }
        if let Some(d2) = delta.get_mut(&m.b) {
            *d2 += part_d(mu2, mu1, phi1, s2);
        }
    }

    // take the inverse of v
    for (_, v_i) in v.iter_mut() {
        *v_i = 1.0 / *v_i;
    }
    // multiple d by v
    for (id, d_i) in delta.iter_mut() {
        *d_i *= v[id];
    }

    for c in characters.iter_mut() {
        if !v.contains_key(&c.id) {
            // Convert the ratings and RD’s onto the Glicko-1 scale
            c.rank.glicko_2_to_1_scale();
            continue;
        }
        // Determine the new value of the volatility, deviation, and rating
        c.rank.vola = new_volatility(
            v[&c.id],
            delta[&c.id],
            c.rank.vola,
            c.rank.devi,
            TAU,
            EPSILON,
        );
        c.rank.devi = new_deviation(c.rank.devi, c.rank.vola, v[&c.id]);
        c.rank.rati = new_rating(c.rank.rati, c.rank.devi, v[&c.id], delta[&c.id]);

        // Convert the ratings and RD’s onto the Glicko-1 scale
        c.rank.glicko_2_to_1_scale();
    }
}

pub fn calculate_ranking(characters: &[Character]) -> (Vec<Character>, HashMap<usize, usize>) {
    let mut list = Vec::from(characters);
    list.sort_by(|a, b| {
        if a.rank.rati != b.rank.rati {
            b.rank.rati.total_cmp(&a.rank.rati)
        } else if a.rank.devi != b.rank.devi {
            b.rank.devi.total_cmp(&a.rank.devi)
        } else {
            b.id.cmp(&a.id)
        }
    });

    let mut ranks: HashMap<usize, usize> = HashMap::with_capacity(characters.len());
    let mut rank = 1;
    let mut max_rating = list[0].rank.rati;
    for c in list.iter() {
        if c.rank.rati < max_rating {
            rank += 1;
            max_rating = c.rank.rati;
        }
        ranks.insert(c.id, rank);
    }

    (list, ranks)
}

pub fn update_history(
    characters: &mut Vec<Character>,
    records: &[Match],
    ranks: &HashMap<usize, usize>,
) {
    for m in records.iter() {
        match m.res {
            MatchResult::AWin => {
                characters[m.a].hist.wins += 1;
                characters[m.b].hist.loss += 1;
            }
            MatchResult::BWin => {
                characters[m.a].hist.loss += 1;
                characters[m.b].hist.wins += 1;
            }
            MatchResult::Draw => {
                characters[m.a].hist.draw += 1;
                characters[m.b].hist.draw += 1;
            }
            MatchResult::BothLose => {
                characters[m.a].hist.loss += 1;
                characters[m.b].hist.loss += 1;
            }
        };

        let name_a = characters[m.a].name.clone();
        let name_b = characters[m.b].name.clone();
        let (res_a, res_b) = match m.res {
            MatchResult::AWin => (MatchResult::AWin, MatchResult::BWin),
            MatchResult::BWin => (MatchResult::BWin, MatchResult::AWin),
            MatchResult::Draw => (MatchResult::Draw, MatchResult::Draw),
            MatchResult::BothLose => (MatchResult::BothLose, MatchResult::BothLose),
        };
        characters[m.a]
            .hist
            .recent
            .push_back(Battle::new(name_b, res_a));
        characters[m.b]
            .hist
            .recent
            .push_back(Battle::new(name_a, res_b));
        while characters[m.a].hist.recent.len() > MAX_HIST {
            characters[m.a].hist.recent.pop_front();
        }
        while characters[m.b].hist.recent.len() > MAX_HIST {
            characters[m.b].hist.recent.pop_front();
        }
    }
    for c in characters.iter_mut() {
        c.hist.old_rate.push_back(c.rank.rati);
        c.hist.old_rank.push_back(ranks[&c.id]);
        while c.hist.old_rate.len() > MAX_HIST {
            c.hist.old_rate.pop_front();
            c.hist.old_rank.pop_front();
        }
    }
}

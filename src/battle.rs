use crate::structs::Character;

use rand::prelude::*;
use rand::distributions::WeightedIndex;

pub fn pick_2_player_ids(pool: &[Character]) -> Vec<usize> {
    // Calculate the weights (inverse of the battles)
    let weights: Vec<_> = pool.iter().map(|c| {
        // Adding 1 to avoid division by zero
        1.0 / (c.hist.battles() as f64 + 1.0)
    }).collect();

    // Create a weighted index distribution
    let distribution = WeightedIndex::new(&weights).unwrap();
    let mut rng = thread_rng();

    // Select two distinct indices
    let first_index = distribution.sample(&mut rng);
    let mut second_index = distribution.sample(&mut rng);
    while second_index == first_index {
        second_index = distribution.sample(&mut rng);
    }

    vec![first_index, second_index]
}

use std::collections::{HashMap, HashSet};

use crate::{Solution, SolutionPair};

const PRUNE: i64 = 16777216;

fn mutate(mut s: i64) -> i64 {
    s = (s ^ (s * 64)) % PRUNE;
    s = (s ^ (s / 32)) % PRUNE;
    s = (s ^ (s * 2048)) % PRUNE;
    s
}

pub fn solve(str: String) -> SolutionPair {
    let secret: Vec<i64> = str.lines().map(|v| v.parse().unwrap()).collect();

    let sol1: i64 = secret
        .iter()
        .map(|s| {
            let mut s = *s;
            for _ in 0..2000 {
                s = mutate(s);
            }

            s
        })
        .sum();

    // Find all possible difference sequences and their resulting prices
    let mut score_map: HashMap<[i8; 4], i64> = HashMap::new();
    for s in secret {
        let mut seen: HashSet<[i8; 4]> = HashSet::new();
        let mut vals: [i8; 4] = [0; 4];

        // Initialize first three values in seen values
        let mut s = s;
        for i in 0..3 {
            let new = mutate(s);
            let diff = (new % 10) - (s % 10);
            vals[i + 1] = diff as i8;
            s = new;
        }

        // Using the four differences as a key, find price for each sequence
        for _ in 3..2000 {
            let new = mutate(s);
            let diff = (new % 10) - (s % 10);
            for i in 0..3 {
                vals[i] = vals[i + 1];
            }

            vals[3] = diff as i8;
            let price = new % 10;

            if !seen.contains(&vals) {
                seen.insert(vals);
                score_map
                    .entry(vals)
                    .and_modify(|score| *score += price)
                    .or_insert(price);
            }

            s = new;
        }
    }

    let sol2 = *score_map.values().max().unwrap();

    (Solution::from(sol1), Solution::from(sol2))
}

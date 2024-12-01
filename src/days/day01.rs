use std::collections::HashMap;

use crate::{Solution, SolutionPair};

pub fn solve(str: String) -> SolutionPair {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let mut right_map: HashMap<i32, i32> = HashMap::new();

    for line in str.lines() {
        let mut iter = line.split_ascii_whitespace();
        left.push(iter.next().unwrap().parse::<i32>().unwrap());
        let right_val = iter.next().unwrap().parse::<i32>().unwrap();
        right.push(right_val);
        *right_map.entry(right_val).or_insert(0) += 1;
    }

    left.sort();
    right.sort();

    let sol1: u32 = left
        .iter()
        .zip(right.iter())
        .map(|(&a, &b)| i32::abs_diff(a, b))
        .sum();

    let sol2: i32 = left
        .iter()
        .map(|&v| v * right_map.get(&v).cloned().unwrap_or_default())
        .sum();

    (Solution::from(sol1), Solution::from(sol2))
}

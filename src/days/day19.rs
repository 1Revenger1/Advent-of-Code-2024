use std::collections::{HashMap, HashSet};

use crate::{Solution, SolutionPair};

fn p1<'a>(p: &'a str, cache: &mut HashMap<&'a str, bool>, towels: &HashSet<&str>, max_size: usize) -> bool {
    if p.len() == 0 {
        return true;
    }

    if cache.contains_key(p) {
        return cache[p];
    }

    for i in (1..=p.len().min(max_size)).rev() {
        if towels.contains(&p[0..i]) && p1(&p[i..], cache, towels, max_size) {
            cache.insert(p, true);
            return true;
        }
    }

    cache.insert(p, false);
    false
}

fn p2<'a>(p: &'a str, cache: &mut HashMap<&'a str, usize>, towels: &HashSet<&str>, max_size: usize) -> usize {
    if p.len() == 0 {
        return 1;
    }

    if cache.contains_key(p) {
        return cache[p];
    }

    let mut sum = 0;
    for i in (1..=p.len().min(max_size)).rev() {
        if towels.contains(&p[0..i]) {
            sum += p2(&p[i..], cache, towels, max_size);
        }
    }

    cache.insert(p, sum);
    sum
}

pub fn solve(str: String) -> SolutionPair {
    let (towels, patterns) = str.split_once("\n\n").unwrap();
    let towels: HashSet<&str> = towels.split(',').map(|t| t.trim()).collect();

    let max_size = towels.iter().map(|t| t.len()).max().unwrap();
    let mut cache = HashMap::new();

    let sol1 = patterns.lines().filter(|p| {
        p1(p, &mut cache, &towels, max_size)
    })
    .count();

    let mut cache = HashMap::new();
    let sol2: usize = patterns.lines().map(|p| p2(p, &mut cache, &towels, max_size)).sum();

    (Solution::from(sol1), Solution::from(sol2))
}

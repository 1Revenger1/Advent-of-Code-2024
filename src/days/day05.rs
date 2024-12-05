use std::collections::{HashMap, HashSet};

use crate::{Solution, SolutionPair};

pub fn solve(str: String) -> SolutionPair {
    let (rules_str, page_nums) = str.split_once("\n\n").unwrap();

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    for l in rules_str.lines() {
        let num = l.split_once('|').unwrap();
        let key: i32 = num.0.parse().unwrap();
        let val: i32 = num.1.parse().unwrap();
        rules.entry(key).or_default().push(val);
    };

    let valid_updates_sum: i32 = page_nums.lines().filter_map(|l| -> Option<i32> {
        let mut seen: HashSet<i32> = HashSet::new();
        let pages: Vec<i32> = l.split(',').map(|p| p.parse().unwrap()).collect();

        for p in &pages {
            let must_be_after = rules.get(&p);
            if let Some(arr) = must_be_after {
                if arr.iter().any(|a| seen.contains(a)) {
                    return None;
                }
            }
            seen.insert(*p);
        }

        Some(pages.get(pages.len() / 2).unwrap().clone())
    }).sum();

    let invalid_updates_sum: i32 = page_nums.lines().filter_map(|l| {
        let mut seen: HashSet<i32> = HashSet::new();
        let pages: Vec<i32> = l.split(',').map(|p| p.parse().unwrap()).collect();
    
        for p in &pages {
            let rule = rules.get(&p);

            if let None = rule {
                seen.insert(*p);
                continue;
            }

            if rule.unwrap().iter().any(|a| seen.contains(a)) {
                return Some(pages);
            }

            seen.insert(*p);
        }

        None
    }).map(|mut update| {
        update.sort_by(|a, b| {
            let rule = rules.get(&a);
            if let None = rule {
                return std::cmp::Ordering::Equal;
            }

            let rule = rule.unwrap();
            if rule.contains(b) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });

        update[update.len() / 2]
    }).sum();

    (Solution::from(valid_updates_sum), Solution::from(invalid_updates_sum))
}

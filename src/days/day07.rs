use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve(str: String) -> SolutionPair {
    let equations: Vec<(u64, Vec<u64>)> = str
        .lines()
        .map(|l| {
            let (val, operands) = l.split_once(':').unwrap();
            let val = val.parse().unwrap();
            let operands = operands.split_ascii_whitespace().map(|v| v.parse().unwrap()).collect();
            (val, operands)
        })
        .collect();

    let sol1: u64 = equations
        .iter()
        .map(|(val, operands)| {
            let num_operators = operands.len() - 1;
            for perm in 0..=((1 << num_operators) - 1) {
                let mut res = operands[0];
                for (bit, v) in operands.iter().skip(1).enumerate() {
                    if (perm & (1 << bit)) != 0 {
                        res *= *v;
                    } else {
                        res += v;
                    }
                }

                if res == *val {
                    return *val;
                }
            }
            0
        })
        .sum();

    let sol2: u64 = equations
        .iter()
        .map(|(val, operands)| {
            let num_operators = operands.len() - 1;
            'next_perm: for perm in 0..=((1 << (num_operators * 2)) - 1) {
                let mut res = operands[0];
                for (bit, v) in operands.iter().skip(1).enumerate() {
                    let op = (perm >> (bit * 2)) & 0x3;
                    match op {
                        0 => res += v,
                        1 => res *= *v,
                        2 => res = (res * 10_u64.pow(v.ilog10() + 1)) + *v,
                        _ => continue 'next_perm
                    }
                }

                if res == *val {
                    return *val;
                }
            }
            0
        })
        .sum();

    (Solution::from(sol1), Solution::from(sol2))
}

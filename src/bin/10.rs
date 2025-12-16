advent_of_code::solution!(10);

use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u64> {
    let re = Regex::new(r"\[([^\]]*)\]|\(([^)]*)\)").unwrap();

    Some(
        input
            .par_lines()
            .filter(|l| !l.is_empty())
            .map(|line| {
                let mut target = 0u16;
                let mut buttons = Vec::new();

                for caps in re.captures_iter(line) {
                    if let Some(m) = caps.get(1) {
                        // target
                        for (i, b) in m.as_str().bytes().enumerate() {
                            if b == b'#' {
                                target |= 1 << i;
                            }
                        }
                    } else if let Some(m) = caps.get(2) {
                        // button
                        let mut mask = 0u16;
                        for n in m.as_str().split(',') {
                            let idx: u16 = n.parse().unwrap();
                            mask |= 1 << idx;
                        }
                        buttons.push(mask);
                    }
                }

                let mut best = u64::MAX;

                for subset in 0..(1 << buttons.len()) {
                    let mut acc = 0u16;
                    let mut presses = 0;

                    for (i, button) in buttons.iter().enumerate() {
                        if (subset >> i) & 1 == 1 {
                            acc ^= *button;
                            presses += 1;
                        }
                    }

                    if acc == target {
                        best = best.min(presses);
                    }
                }

                best
            })
            .sum(),
    )
}

fn solve_recursive(joltages: Vec<u64>, buttons: &[u16], memo: &mut HashMap<Vec<u64>, u64>) -> u64 {
    if let Some(&cached) = memo.get(&joltages) {
        return cached;
    }

    if joltages.iter().all(|&j| j == 0) {
        return 0;
    }

    let mut target_parity = 0u16;
    for (i, &j) in joltages.iter().enumerate() {
        if j % 2 == 1 {
            target_parity |= 1 << i;
        }
    }

    let mut min_cost = u64::MAX;

    for i in 0..(1 << buttons.len()) {
        let mut subset_presses = 0;
        let mut subset_mask = 0u16;
        let mut cost: u64 = 0;

        for (j, button) in buttons.iter().enumerate() {
            if (i >> j) & 1 == 1 {
                subset_presses |= 1 << j;
                subset_mask ^= *button;
                cost += 1;
            }
        }

        if subset_mask == target_parity {
            let mut next_joltages = joltages.clone();
            let mut possible = true;

            for (j, button) in buttons.iter().enumerate() {
                if (subset_presses >> j) & 1 == 1 {
                    for (k, joltage) in next_joltages.iter_mut().enumerate().take(joltages.len()) {
                        if (*button >> k) & 1 == 1 {
                            if *joltage == 0 {
                                possible = false;
                                break;
                            }
                            *joltage -= 1;
                        }
                    }
                }
                if !possible {
                    break;
                }
            }

            if possible && next_joltages.iter().all(|&j| j % 2 == 0) {
                for j in next_joltages.iter_mut() {
                    *j /= 2;
                }
                let sub_cost = solve_recursive(next_joltages, buttons, memo);
                if sub_cost != u64::MAX {
                    min_cost = min_cost.min(cost + 2 * sub_cost);
                }
            }
        }
    }

    memo.insert(joltages, min_cost);
    min_cost
}

pub fn part_two(input: &str) -> Option<u64> {
    let re = Regex::new(r"\(([^)]*)\)|\{([^\}]*)\}").unwrap();
    Some(
        input
            .par_lines()
            .filter(|l| !l.is_empty())
            .map(|line| {
                let mut buttons = Vec::new();
                let mut joltages = Vec::new();

                for caps in re.captures_iter(line) {
                    if let Some(m) = caps.get(1) {
                        // button
                        let mut mask = 0u16;
                        for n in m.as_str().split(',') {
                            if let Ok(idx) = n.parse::<u16>() {
                                mask |= 1 << idx;
                            }
                        }
                        buttons.push(mask);
                    } else if let Some(m) = caps.get(2) {
                        // joltages
                        joltages = m
                            .as_str()
                            .split(',')
                            .filter_map(|s| s.parse().ok())
                            .collect();
                    }
                }

                if joltages.is_empty() {
                    return 0;
                }

                let mut memo = HashMap::new();
                solve_recursive(joltages, &buttons, &mut memo)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}

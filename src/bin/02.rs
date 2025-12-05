use std::collections::HashSet;
advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut count = 0;

    for cell in input.trim().split(',').filter(|c| !c.is_empty()) {
        let mut ids = cell.split('-').filter_map(|s| s.parse::<u64>().ok());

        let Some(start) = ids.next() else { continue };
        let Some(end) = ids.next() else { continue };

        if start > end {
            continue;
        }

        for n in start..=end {
            let s = n.to_string();
            if does_contains_substring_twice(&s) {
                count += n;
            }
        }
    }
    Some(count)
}

fn does_contains_substring_twice(s: &str) -> bool {
    let len = s.len();
    len % 2 == 0 && &s[..len / 2] == &s[len / 2..]
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut count = 0;
    let mut unique_numbers = HashSet::new();
    for cell in input.trim().split(',').filter(|c| !c.is_empty()) {
        let mut ids = cell.split('-').filter_map(|s| s.parse::<u64>().ok());

        let Some(mut start) = ids.next() else { continue };
        let Some(mut end) = ids.next() else { continue };

        if start > end {
           (start, end) = (end, start);
        }

        for n in start..=end {
            let s = n.to_string();
            if does_contain_repeat_substring(&s) && unique_numbers.insert(n) {
                count += n;
            }
        }
    }
    Some(count)
}

fn does_contain_repeat_substring(input: &str) -> bool {
    let len = input.len();

    for sub_len in 1..=len / 2 {
        let sub = &input[..sub_len];
        let remainder = &input[sub_len..];

        if remainder.len() % sub_len != 0 {
            continue;
        }

        let expected = remainder.len() / sub_len;

        if (0..expected).all(|i| &remainder[i * sub_len..(i + 1) * sub_len] == sub) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}

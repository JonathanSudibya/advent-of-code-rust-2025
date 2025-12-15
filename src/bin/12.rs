advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    let count = input
        .lines()
        .filter(|line| line.contains('x'))
        .filter_map(|line| {
            let (dims, quantities_str) = line.split_once(": ")?;
            let (w_str, h_str) = dims.split_once('x')?;
            let w: u64 = w_str.parse().ok()?;
            let h: u64 = h_str.parse().ok()?;

            let num_presents: u64 = quantities_str
                .split_whitespace()
                .filter_map(|s| s.parse::<u64>().ok())
                .sum();

            if (w * h / 7) > num_presents {
                Some(1)
            } else {
                Some(0)
            }
        })
        .sum();
    Some(count)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

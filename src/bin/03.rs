advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut joltage = 0;
    for line in input.split('\n').filter(|c| !c.is_empty()) {
        let mut largest_char = line.chars().nth(0).unwrap();
        let mut second_largest_char = line.chars().nth(1).unwrap();
        let mut second_largest_index = 1;
        for n in 2..line.len() {
            let c = line.chars().nth(n).unwrap();
            if c > largest_char && n < line.len()-1 {
                largest_char = c;
                if second_largest_index < n {
                    second_largest_char = line.chars().nth(n+1).unwrap();
                    second_largest_index = n+1;
                }
                continue;
            }
            if c > second_largest_char {
                second_largest_char = c;
            }
        }

        let line_joltage = format!("{}{}", largest_char, second_largest_char).parse::<u64>().unwrap();
        joltage += line_joltage;
    }
    Some(joltage)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut joltage = 0;
    for line in input.split('\n').filter(|c| !c.is_empty()) {
        let selected_indexes = [0,1,2,3,4,5,6,7,8,9,10,11];
        let mut largest_char = line.chars().nth(0).unwrap();
        let mut second_largest_char = line.chars().nth(1).unwrap();
        let mut second_largest_index = 1;
        for n in 2..line.len() {
            let c = line.chars().nth(n).unwrap();
            if c > largest_char && n < line.len()-1 {
                largest_char = c;
                if second_largest_index < n {
                    second_largest_char = line.chars().nth(n+1).unwrap();
                    second_largest_index = n+1;
                }
                continue;
            }
            if c > second_largest_char {
                second_largest_char = c;
            }
        }

        let line_joltage = format!("{}{}", largest_char, second_largest_char).parse::<u64>().unwrap();
        joltage += line_joltage;
    }
    Some(joltage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut number_table = vec![vec![0; 1000]; 1000];
    let mut operators = vec![0; 1000];
    let mut row_count = 0;
    let mut total_group = 0;
    for line in input.split('\n').filter(|c| !c.is_empty()) {
        let mut group_count = 0;
        for word in line.split_whitespace() {
            match word {
                "*" => operators[group_count] = word.as_bytes()[0],
                "+" => operators[group_count] = word.as_bytes()[0],
                _ => {
                    let parsed_number = word.parse::<u64>().unwrap();
                    number_table[group_count][row_count] = parsed_number;
                }
            }
            group_count += 1;
        }
        if total_group == 0 {
            total_group = group_count;
        }
        row_count += 1;
    }
    row_count -= 1;
    let mut result = 0;
    for n in 0..total_group {
        let op = operators[n];
        match op {
            42 => {
                // '*'
                let mut sub_total = 0;
                for i in 0..row_count {
                    let number = number_table[n][i];
                    if sub_total == 0 {
                        sub_total += number
                    } else {
                        sub_total *= number
                    }
                }
                result += sub_total;
            }
            43 => {
                // '+'
                for i in 0..row_count {
                    result += number_table[n][i];
                }
            }
            _ => {
                continue;
            }
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

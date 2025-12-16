use std::str::from_utf8;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut joltage = 0;
    for line in input.split('\n').filter(|c| !c.is_empty()) {
        let line_bytes = line.as_bytes();
        let mut selected_indexes = [0, 1];
        let mut search_range = line_bytes.len() - selected_indexes.len();
        for i in 0..selected_indexes.len() {
            let initial_idx = selected_indexes[i];
            let mut initial_byte = line_bytes[initial_idx];
            let max_index = initial_idx + search_range;
            for (n, _) in line_bytes.iter().enumerate().take(max_index + 1).skip(initial_idx + 1) {
                if line_bytes[n] > initial_byte {
                    initial_byte = line_bytes[n];
                    selected_indexes[i] = n
                }
            }
            let move_index = selected_indexes[i] - initial_idx;
            if move_index == 0 {
                continue;
            }
            for selected_index in selected_indexes.iter_mut().skip(i+1) {
                *selected_index += move_index;
            }
            search_range -= move_index;
            if search_range == 0 {
                break;
            }
        }
        let mut selected_bytes: [u8; 2] = [0; 2];
        for n in 0..selected_indexes.len() {
            selected_bytes[n] = line_bytes[selected_indexes[n]];
        }
        let selected_u64 = from_utf8(&selected_bytes).unwrap().parse::<u64>().unwrap();
        joltage += selected_u64;
    }
    Some(joltage)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut joltage = 0;
    for line in input.split('\n').filter(|c| !c.is_empty()) {
        let line_bytes = line.as_bytes();
        let mut selected_indexes = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let mut search_range = line_bytes.len() - selected_indexes.len();
        for i in 0..selected_indexes.len() {
            let initial_idx = selected_indexes[i];
            let mut initial_byte = line_bytes[initial_idx];
            let max_index = initial_idx + search_range;
            for (n, selected_byte) in line_bytes.iter().enumerate().take(max_index + 1).skip(initial_idx + 1) {
                if *selected_byte > initial_byte {
                    initial_byte = *selected_byte;
                    selected_indexes[i] = n
                }
            }
            let move_index = selected_indexes[i] - initial_idx;
            if move_index == 0 {
                continue;
            }
            for selected_index in selected_indexes.iter_mut().skip(i + 1) {
                *selected_index += move_index;
            }
            search_range -= move_index;
            if search_range == 0 {
                break;
            }
        }
        let mut selected_bytes: [u8; 12] = [0; 12];
        for n in 0..selected_indexes.len() {
            selected_bytes[n] = line_bytes[selected_indexes[n]];
        }
        let selected_u64 = from_utf8(&selected_bytes).unwrap().parse::<u64>().unwrap();
        joltage += selected_u64;
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
        assert_eq!(result, Some(3121910778619));
    }
}

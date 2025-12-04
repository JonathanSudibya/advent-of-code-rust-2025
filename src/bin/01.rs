advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lock_position: u8 = 50;
    let mut actual_password = 0;
    let input_string = input.to_string();
    let input_lines = input_string.split("\n");
    for line in input_lines {
        if line.len() == 0 {
            continue;
        }
        let direction = line.chars().nth(0).unwrap();
        let move_string = line.to_string();
        let move_size = (move_string[1..].parse::<i32>().unwrap() % 100) as u8;
        if direction == 'L' {
            if lock_position < move_size {
                lock_position = 100 - (move_size - lock_position);
            } else {
                lock_position -= move_size;
            }
        } else if direction == 'R' {
            if lock_position + move_size > 99 {
                lock_position = lock_position + move_size - 100;
            } else {
                lock_position += move_size;
            }
        }

        if lock_position == 0 {
            actual_password += 1;
        }
    }
    Some(actual_password)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lock_position: i32 = 50;
    let mut actual_password = 0;
    let input_string = input.to_string();
    let input_lines = input_string.split("\n");
    for line in input_lines {
        if line.len() == 0 {
            continue;
        }
        let direction = line.chars().nth(0).unwrap();
        let move_string = line.to_string();
        let move_int = move_string[1..].parse::<i32>().unwrap();
        let move_size = move_int % 100;
        let move_rotation = (move_int / 100) as u64;
        let previous_lock_position = lock_position;
        if direction == 'L' {
            lock_position -= move_size
        } else if direction == 'R' {
            lock_position += move_size
        }

        if lock_position < 0 {
            lock_position += 100;
            if previous_lock_position != 0 {
                actual_password += 1;
            }
        } else if lock_position >= 100 {
            lock_position -= 100;
            if previous_lock_position != 0 {
                actual_password += 1;
            }
        } else if lock_position == 0 {
            if previous_lock_position != 0 {
                actual_password += 1;
            }
        }

        actual_password += move_rotation
    }
    Some(actual_password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}

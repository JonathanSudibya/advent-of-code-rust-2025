advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lock_position= 50;
    let mut actual_password = 0;
    for line in input.lines().filter(|c| !c.is_empty())  {
        let line_bytes = line.as_bytes();
        let direction = line_bytes[0];
        let move_size = str::from_utf8(&line_bytes[1..]).unwrap().parse::<u64>().unwrap() % 100;
        if direction == b'L' {
            if lock_position < move_size {
                lock_position = 100 - (move_size - lock_position);
            } else {
                lock_position -= move_size;
            }
        } else if direction == b'R' {
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
    for line in input.lines().filter(|c| !c.is_empty()) {
        let line_bytes = line.as_bytes();
        let direction = line_bytes[0];
        let move_int = str::from_utf8(&line_bytes[1..]).unwrap().parse::<i32>().unwrap();
        let move_size = move_int % 100;
        let move_rotation = (move_int / 100) as u64;
        let previous_lock_position = lock_position;
        if direction == b'L' {
            lock_position -= move_size
        } else if direction == b'R' {
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
        } else if lock_position == 0
            && previous_lock_position != 0 {
                actual_password += 1;
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

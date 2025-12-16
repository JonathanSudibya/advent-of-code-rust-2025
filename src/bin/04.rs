advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let mut field_data: Vec<Vec<u8>> = Vec::new();
    let mut row_length = 0;
    for line in input.split('\n').filter(|c| !c.is_empty()) {
        field_data.push(line.as_bytes().to_vec());
        row_length = line.len()
    }
    let column_length = field_data.len();
    let mut paper_roll_removed = 0;
    for y in 0..column_length {
        for x in 0..row_length {
            let b = field_data[y][x];
            if b != b'@' {
                continue;
            }

            let neighbouring_paper_roll_count =
                check_neighbour_count(&field_data, x, y, row_length, column_length);

            if neighbouring_paper_roll_count < 4 {
                paper_roll_removed += 1;
            }
        }
    }
    Some(paper_roll_removed)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut field_data: Vec<Vec<u8>> = Vec::new();
    let mut row_length = 0;
    for line in input.split('\n').filter(|c| !c.is_empty()) {
        field_data.push(line.as_bytes().to_vec());
        row_length = line.len()
    }
    let column_length = field_data.len();
    let mut sum_paper_roll_removed = 0;
    let mut paper_roll_removed = 0;
    loop {
        let mut marked_paper_roll: Vec<(usize, usize)> = Vec::new();
        for y in 0..column_length {
            for x in 0..row_length {
                let b = field_data[y][x];
                if b != b'@' {
                    continue;
                }

                let neighbouring_paper_roll_count =
                    check_neighbour_count(&field_data, x, y, row_length, column_length);

                if neighbouring_paper_roll_count < 4 {
                    paper_roll_removed += 1;
                    marked_paper_roll.push((x, y));
                }
            }
        }

        for (x, y) in marked_paper_roll {
            field_data[y][x] = b'.'
        }

        sum_paper_roll_removed += paper_roll_removed;

        if paper_roll_removed == 0 {
            break;
        } else {
            paper_roll_removed = 0
        }
    }

    Some(sum_paper_roll_removed)
}

fn check_neighbour_count(
    field_data: &[Vec<u8>],
    x: usize,
    y: usize,
    row_length: usize,
    column_length: usize,
) -> u64 {
    let b = field_data[y][x];

    // check for neighbouring
    let mut neighbouring_paper_roll_count = 0;
    if (y > 0 && x > 0) && field_data[y - 1][x - 1] == b {
        neighbouring_paper_roll_count += 1;
    }
    if (y > 0) && field_data[y - 1][x] == b {
        neighbouring_paper_roll_count += 1;
    }
    if (y > 0 && x < (row_length - 1)) && field_data[y - 1][x + 1] == b {
        neighbouring_paper_roll_count += 1;
    }
    if (x > 0) && field_data[y][x - 1] == b {
        neighbouring_paper_roll_count += 1;
    }
    // if field_data[y][x] == b {
    //     neighbouring_paper_roll_count += 1;
    // }
    if (x < (row_length - 1)) && field_data[y][x + 1] == b {
        neighbouring_paper_roll_count += 1;
    }
    if (y < (column_length - 1) && x > 0) && field_data[y + 1][x - 1] == b {
        neighbouring_paper_roll_count += 1;
    }
    if (y < (column_length - 1)) && field_data[y + 1][x] == b {
        neighbouring_paper_roll_count += 1;
    }
    if (y < (column_length - 1) && x < (row_length - 1)) && field_data[y + 1][x + 1] == b {
        neighbouring_paper_roll_count += 1;
    }

    neighbouring_paper_roll_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}

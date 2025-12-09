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
    let mut vec_table = vec![vec![0u8;4000];4000];
    let mut line_count = 0;
    for line in input.split('\n').filter(|c| !c.is_empty()) {
        vec_table[line_count] = line.as_bytes().to_vec();
        vec_table[line_count].reverse();
        line_count += 1;
    }
    let num_digit_rows = line_count - 1; // operator is on the last line
    vec_table = transpose(&vec_table);
    let mut tmp_vec: Vec<u64> = Vec::new();
    let mut result = 0;
    for line in vec_table {
        // Try to parse the number part (everything except the last element which is the operator)
        let number_str = str::from_utf8(&line[..num_digit_rows]).unwrap();

        // Get the operator (last element)
        let op = line[num_digit_rows] as char;

        // Try to parse the number; skip if it's all spaces (separator column)
        if let Ok(number) = number_str.trim().parse::<u64>() {
            tmp_vec.push(number);
        }

        // Process when we hit an operator
        match op {
            '+' => {
                for n in &tmp_vec {
                    result += n;
                }
                tmp_vec = Vec::new();
            }
            '*' => {
                let mut sub_total = 0;
                for n in &tmp_vec {
                    if sub_total == 0 {
                        sub_total += n;
                    } else {
                        sub_total *= n;
                    }
                }
                result += sub_total;
                tmp_vec = Vec::new();
            }
            _ => {
                // Space means either separator or part of current problem
                continue;
            }
        }
    }

    Some(result)
}

fn transpose(matrix: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    // Handle the case of an empty matrix or a matrix with empty rows
    if matrix.is_empty() || matrix[0].is_empty() {
        return Vec::new();
    }

    let num_rows = matrix.len();
    let num_cols = matrix[0].len();

    // Create a new matrix with dimensions swapped (cols x rows)
    let mut transposed_matrix: Vec<Vec<u8>> = vec![vec![0; num_rows]; num_cols];

    // Iterate through the original matrix and populate the transposed matrix
    for r in 0..num_rows {
        for c in 0..num_cols {
            transposed_matrix[c][r] = matrix[r][c];
        }
    }

    transposed_matrix
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
        assert_eq!(result, Some(3263827));
    }
}

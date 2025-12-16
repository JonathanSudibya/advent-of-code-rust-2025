use std::{collections::HashMap};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut table: Vec<Vec<u8>> = Vec::new();
    for line in input.split('\n').filter(|c| !c.is_empty()) {
        table.push(line.as_bytes().to_vec());
    }

    let mut result = 0;

    for i in 0..table.len() {
        for j in 0..table[i].len() {
            if (table[i][j] as char) == 'S' {
                // start downward
                result = search_splitter(i+1, j, &mut table);
                break;
            }
        }
        if result > 0 {
            break;
        }
    }
    Some(result)
}

fn search_splitter(i: usize, j: usize, table: &mut Vec<Vec<u8>>) -> u64 {
    if i >= table.len() {
        return 0
    }
    let c = table[i][j] as char;
    match c {
        '^' => {
            let spliter_count_l: u64 = if j > 0 {
                search_splitter(i, j-1, table)
            } else {0};
            let spliter_count_r: u64 = if j < table[i].len() {
                search_splitter(i, j+1, table)
            }else {0};
            spliter_count_l + spliter_count_r + 1
        }
        '.' => {
            table[i][j] = b'|';
            search_splitter(i+1, j, table)
        }
        _ => {
            0
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut table: Vec<Vec<u8>> = Vec::new();
    for line in input.split('\n').filter(|c| !c.is_empty()) {
        table.push(line.as_bytes().to_vec());
    }

    let mut result = 0;
    let mut dictionary = HashMap::new();

    for i in 0..table.len() {
        for j in 0..table[i].len() {
            if (table[i][j] as char) == 'S' {
                // start downward
                result = search_paths(i+1, j, &table, &mut dictionary);
                break;
            }
        }
        if result > 0 {
            break;
        }
    }
    Some(result)
}

fn search_paths(i: usize, j: usize, table: &Vec<Vec<u8>>, dictionary: &mut HashMap<String, u64>) -> u64 {
    if i >= table.len() {
        return 1
    }
    let c = table[i][j] as char;
    let key = format!("{},{}", i, j);
    if let Some(value) = dictionary.get(&key) {
        return *value;
    }
    match c {
        '^' => {
            let spliter_count_l: u64 = if j > 0 {
                search_paths(i, j-1, table, dictionary)
            } else {0};
            let spliter_count_r: u64 = if j < table[i].len() {
                search_paths(i, j+1, table, dictionary)
            }else {0};
            let sub_total = spliter_count_l + spliter_count_r;
            dictionary.insert(key, sub_total);
            sub_total
        }
        '.' => {
            search_paths(i+1, j, table, dictionary)
        }
        _ => {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}

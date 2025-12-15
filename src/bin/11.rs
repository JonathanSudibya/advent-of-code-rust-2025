use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let mut path_dictionary = HashMap::new();
    for line in input.lines().filter(|c| !c.is_empty()) {
        let line_split: Vec<&str> = line.split(':').collect();
        let target_split: Vec<&str> = line_split[1].split_whitespace().collect();
        path_dictionary.insert(line_split[0], target_split);
    }

    let paths = search_paths("you", &path_dictionary);
    Some(paths)
}

fn search_paths(target: &str, path_dictionary: &HashMap<&str, Vec<&str>>) -> u64 {
    if target == "out" {
        return 1;
    }

    let mut count = 0;
    for t in path_dictionary.get(target).unwrap() {
        count += search_paths(t, path_dictionary)
    }
    count
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut path_dictionary = HashMap::new();
    for line in input.lines().filter(|c| !c.is_empty()) {
        let line_split: Vec<&str> = line.split(':').collect();
        let target_split: Vec<&str> = line_split[1].split_whitespace().collect();
        path_dictionary.insert(line_split[0], target_split);
    }

    let mut memoization = HashMap::new();
    let paths = search_paths_server("svr", &path_dictionary, false, false, &mut memoization);
    Some(paths)
}

fn search_paths_server<'a>(
    target: &'a str,
    path_dictionary: &HashMap<&str, Vec<&'a str>>,
    dac_visited: bool,
    fft_visited: bool,
    memo: &mut HashMap<(&'a str, bool, bool), u64>,
) -> u64 {
    let key = (target, dac_visited, fft_visited);
    if let Some(count) = memo.get(&key) {
        return *count;
    }

    if target == "out" {
        if dac_visited && fft_visited {
            return 1;
        }
        return 0;
    }

    let mut count = 0;
    for t in path_dictionary.get(target).unwrap() {
        let dac_v = if *t == "dac" { true } else { dac_visited };
        let fft_v = if *t == "fft" { true } else { fft_visited };
        let tmp_count = search_paths_server(t, path_dictionary, dac_v, fft_v, memo);
        count += tmp_count;
    }
    memo.insert(key, count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(2));
    }
}

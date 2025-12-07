advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    // creating db
    let mut fresh_items = 0;
    let (freshness_range, item_ids) = create_database(input);
    for id in item_ids {
        for (start_id, end_id) in &freshness_range {
            if start_id <= &id && &id <= end_id {
                fresh_items += 1;
                break;
            };
        }
    }
    Some(fresh_items)
}

pub fn part_two(input: &str) -> Option<u64> {
    // creating db
    let mut fresh_items = 0;
    let (mut freshness_range, _) = create_database(input);
    let mut range_items: Vec<(u64, u64)> = Vec::new();

    freshness_range.sort_by(|a, b| a.0.cmp(&b.0));

    for (start_id, end_id) in &freshness_range {
        let mut new_start_block = *start_id;
        let mut new_end_block = *end_id;
        for (old_start_id, old_end_id) in &range_items {
            if *old_start_id <= new_start_block && new_start_block <= *old_end_id {
                new_start_block = *old_end_id + 1;
            }
            if *old_start_id <= new_end_block && new_end_block <= *old_end_id {
                new_end_block = *old_start_id - 1;
            }
        }

        if new_end_block >= new_start_block {
            range_items.push((new_start_block, new_end_block));
        }
    }

    for (start_id, end_id) in range_items {
        fresh_items += (end_id - start_id) + 1;
    }

    Some(fresh_items)
}

fn create_database(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut freshness_range: Vec<(u64, u64)> = Vec::new();
    let mut item_ids: Vec<u64> = Vec::new();
    for line in input.split('\n').filter(|c| !c.is_empty()) {
        if line.contains('-') {
            let mut first_id = 0;
            let mut second_id = 0;
            for id in line.split('-') {
                let parsed_id = id.parse::<u64>().unwrap();
                if first_id == 0 {
                    first_id = parsed_id
                } else {
                    second_id = parsed_id
                }
            }
            if second_id < first_id {
                (first_id, second_id) = (second_id, first_id)
            }
            freshness_range.push((first_id, second_id));
        } else {
            let parsed_id = line.parse::<u64>().unwrap();
            item_ids.push(parsed_id);
        }
    }
    (freshness_range, item_ids)
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
        assert_eq!(result, Some(14));
    }
}

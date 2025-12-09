use std::cmp::Ordering;
use std::collections::BinaryHeap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let mut point_collection: Vec<Point> = Vec::new();
    for line in input.split('\n').filter(|c| !c.is_empty()) {
        let coordinates: Vec<&str> = line.split(',').collect();
        if coordinates.len() == 3 {
            let p = Point{
                x: coordinates[0].parse::<f64>().unwrap(),
                y: coordinates[1].parse::<f64>().unwrap(),
                z: coordinates[2].parse::<f64>().unwrap()
            };
            point_collection.push(p);
        }
    }

    let mut distance_map = BinaryHeap::new();
    for i in 0..point_collection.len() {
        for j in i+1..point_collection.len() {
            let distance = point_collection[i].distance(&point_collection[j]);
            let s = State{p1: i, p2: j, distance: distance};
            distance_map.push(s);
        }
    }

    let mut circuits: Vec<Vec<usize>> = Vec::new();
    for p in 0..point_collection.len() {
        circuits.push(vec![p]);
    }

    // For small inputs (like the example with 20 nodes), use half the node count
    // For large inputs, use 1000 connections
    let target_connections = if point_collection.len() < 100 {
        point_collection.len() / 2
    } else {
        1000
    };

    let mut connections_attempted = 0;
    while let Some(s) = distance_map.pop() {
        if connections_attempted >= target_connections {
            break;
        }

        // search node
        let mut group_p1 = 0;
        let mut group_p1_size = 0;
        let mut group_p2 = 0;
        let mut group_p2_size = 0;
        let mut group_count = 0;
        for group in &circuits {
            if group.contains(&s.p1) {
                group_p1 = group_count;
                group_p1_size = group.len();
            }
            if group.contains(&s.p2) {
                group_p2 = group_count;
                group_p2_size = group.len();
            }

            group_count += 1;
        }

        connections_attempted += 1;

        if group_p1 == group_p2 {
            // Already in same circuit, but still counts as an attempt
            continue;
        }

        // merge into larger group
        let (larger, smaller) = if group_p1_size >= group_p2_size {
            (group_p1, group_p2)
        } else {
            (group_p2, group_p1)
        };

        // Take ownership of the smaller group's elements
        let mut smaller_group = std::mem::take(&mut circuits[smaller]);
        circuits[larger].append(&mut smaller_group);
        circuits.remove(smaller);
    }

    // Get sizes of all circuits and sort them in descending order
    let mut circuit_sizes: Vec<usize> = circuits.iter().map(|c| c.len()).collect();
    circuit_sizes.sort_by(|a, b| b.cmp(a));

    // Multiply the three largest circuit sizes
    if circuit_sizes.len() >= 3 {
        let result = circuit_sizes[0] * circuit_sizes[1] * circuit_sizes[2];
        Some(result as u64)
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut point_collection: Vec<Point> = Vec::new();
    for line in input.split('\n').filter(|c| !c.is_empty()) {
        let coordinates: Vec<&str> = line.split(',').collect();
        if coordinates.len() == 3 {
            let p = Point{
                x: coordinates[0].parse::<f64>().unwrap(),
                y: coordinates[1].parse::<f64>().unwrap(),
                z: coordinates[2].parse::<f64>().unwrap()
            };
            point_collection.push(p);
        }
    }

    let mut distance_map = BinaryHeap::new();
    for i in 0..point_collection.len() {
        for j in i+1..point_collection.len() {
            let distance = point_collection[i].distance(&point_collection[j]);
            let s = State{p1: i, p2: j, distance: distance};
            distance_map.push(s);
        }
    }

    let mut circuits: Vec<Vec<usize>> = Vec::new();
    for p in 0..point_collection.len() {
        circuits.push(vec![p]);
    }

    while let Some(s) = distance_map.pop() {
        // search node
        let mut group_p1 = 0;
        let mut group_p1_size = 0;
        let mut group_p2 = 0;
        let mut group_p2_size = 0;
        let mut group_count = 0;
        for group in &circuits {
            if group.contains(&s.p1) {
                group_p1 = group_count;
                group_p1_size = group.len();
            }
            if group.contains(&s.p2) {
                group_p2 = group_count;
                group_p2_size = group.len();
            }

            group_count += 1;
        }

        if group_p1 == group_p2 {
            // Already in same circuit, but still counts as an attempt
            continue;
        }

        // merge into larger group
        let (larger, smaller) = if group_p1_size >= group_p2_size {
            (group_p1, group_p2)
        } else {
            (group_p2, group_p1)
        };

        // Take ownership of the smaller group's elements
        let mut smaller_group = std::mem::take(&mut circuits[smaller]);
        circuits[larger].append(&mut smaller_group);
        circuits.remove(smaller);

        if circuits.len() == 1 {
            return Some((point_collection[s.p1].x.round() as u64) * (point_collection[s.p2].x.round() as u64))
        }
    }

    None
}

struct Point {
    x: f64,
    y: f64,
    z: f64
}

impl Point {
    pub fn distance(&self, p2:&Self) -> u64 {
        (
            (self.x - p2.x).powi(2) +
            (self.y - p2.y).powi(2) +
            (self.z - p2.z).powi(2))
            .sqrt().round() as u64
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    p1: usize,
    p2: usize,
    distance: u64
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
            .then_with(|| other.p1.cmp(&self.p1))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}

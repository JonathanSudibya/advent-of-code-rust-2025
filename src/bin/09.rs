advent_of_code::solution!(9);

use geo::{coord, Contains, Coord, LineString, Polygon};

pub fn part_one(input: &str) -> Option<u64> {
    let mut table: Vec<Vec<i64>> = Vec::new();
    for line in input.split('\n').filter(|c| !c.is_empty()) {
        let mut table_row: Vec<i64> = Vec::new();

        for coordinate in line.split(',') {
            let c = coordinate.parse::<i64>().unwrap();
            table_row.push(c);
        }

        table.push(table_row);
    }

    let mut result = 0;
    for i in 0..table.len() {
        for j in i + 1..table.len() {
            let p1 = &table[i];
            let p2 = &table[j];

            let size = ((p1[0] - p2[0]).abs() + 1) * ((p1[1] - p2[1]).abs() + 1);
            if result < size {
                result = size;
            }
        }
    }
    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut coord_table: Vec<Coord> = Vec::new();
    for line in input.split('\n').filter(|c| !c.is_empty()) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut idx = 0;
        for coordinate in line.split(',') {
            let c = coordinate.parse::<f64>().unwrap();
            if idx == 0 {
                x = c;
            } else if idx == 1 {
                y = c;
            }
            idx += 1;
        }

        coord_table.push(coord!{ x: x, y: y});
    }
    
    // Create the closed polygon from the red tiles
    let mut polygon_coords = coord_table.clone();
    if polygon_coords.first() != polygon_coords.last() {
        polygon_coords.push(polygon_coords.first().unwrap().clone());
    }
    let line_string: LineString = polygon_coords.into_iter().collect();

    if !line_string.is_closed() {
        return None
    }
    let polygon: Polygon = Polygon::new(line_string, vec![]);

    // Find the largest rectangle with red corners that is fully contained in the polygon
    let mut max_area = 0u64;
    
    for i in 0..coord_table.len() {
        for j in i+1..coord_table.len() {
            let c1 = &coord_table[i];
            let c2 = &coord_table[j];

            // Calculate the area of the rectangle
            let width = (c1.x - c2.x).abs() + 1.0;
            let height = (c1.y - c2.y).abs() + 1.0;
            let area = (width * height).round() as u64;

            // Calculate rectangle bounds
            let min_x = c1.x.min(c2.x);
            let max_x = c1.x.max(c2.x);
            let min_y = c1.y.min(c2.y);
            let max_y = c1.y.max(c2.y);

            // Check if all four corners of the rectangle are in or on the polygon
            let corners = [
                coord! { x: min_x, y: min_y },
                coord! { x: max_x, y: min_y },
                coord! { x: min_x, y: max_y },
                coord! { x: max_x, y: max_y },
            ];

            let all_corners_valid = corners.iter().all(|corner| {
                polygon.contains(corner) || polygon.exterior().contains(corner)
            });

            if all_corners_valid && area > max_area {
                max_area = area;
            }
        }
    }
    
    Some(max_area)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}

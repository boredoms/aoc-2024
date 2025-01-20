use std::collections::HashMap;

use crate::util::{grid::Grid, point::Point, point::CARDINALS};

pub fn parse(input: &str) -> Grid<u8> {
    Grid::from_str(input)
}

// generate all points at manhattan distance less than distance
struct ManhattanIterator {
    center: Point,
    distance: i64,
    x: i64,
    y: i64,
}

impl Point {
    fn iter(&self, distance: i64) -> ManhattanIterator {
        ManhattanIterator {
            center: *self,
            distance,
            x: 0,
            y: -distance,
        }
    }
}

impl Iterator for ManhattanIterator {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if self.y > self.distance {
            return None;
        }

        let current = Point::new(self.center.x + self.x, self.center.y + self.y);

        self.x += 1;

        if self.x.abs() + self.y.abs() > self.distance {
            self.y += 1;
            self.x = -(self.distance - self.y.abs())
        }

        Some(current)
    }
}

pub fn solve_part_one(input: &Grid<u8>) -> usize {
    let start = input.find(|c| *c == b'S').unwrap();
    let end = input.find(|c| *c == b'E').unwrap();

    let mut current = start;
    let mut distance = 0;

    let mut count = 0;

    let mut visited = Grid::new_with_same_size(&input);
    visited[start] = Some(distance);

    // walk through grid from start
    while current != end {
        distance += 1;

        // assumption: the path is unique
        for d in CARDINALS {
            // don't need to check for validity, because of the border
            if (input[current + d] == b'.' || input[current + d] == b'E')
                && visited[current + d].is_none()
            {
                current += d;
                break;
            }
        }

        visited[current] = Some(distance);

        // check for shortcuts

        // if the distance is < 100 we can't have a fast shortcut
        if distance < 100 {
            continue;
        }

        // assumption: we don't have diagonal shortcuts
        // like: #.
        //       .#

        for d in CARDINALS {
            let candidate = current + d + d;
            if input.in_grid(&candidate)
//                && grid[current + d] == b'#' don't need to check for this
                && visited[candidate].is_some()
            {
                if let Some(distance_previous) = visited[candidate] {
                    if distance - distance_previous - 2 >= 100 {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

pub fn solve_part_two(input: &Grid<u8>) -> usize {
    let grid = input;
    let start = grid.find(|c| *c == b'S').unwrap();
    let end = grid.find(|c| *c == b'E').unwrap();

    let mut current = start;
    let mut distance = 0;

    let mut count = 0;

    let mut visited = Grid::new_with_same_size(&grid);
    visited[start] = Some(distance);

    // let mut dbg_map: HashMap<i64, i64> = HashMap::new();

    // walk through grid from start
    while current != end {
        distance += 1;

        // assumption: the path is unique
        for d in CARDINALS {
            // don't need to check for validity, because of the border
            if (grid[current + d] == b'.' || grid[current + d] == b'E')
                && visited[current + d].is_none()
            {
                current += d;
                break;
            }
        }

        visited[current] = Some(distance);

        // check for shortcuts

        // if the distance is < 100 we can't have a fast shortcut
        if distance < 100 {
            continue;
        }

        // assumption: we don't have diagonal shortcuts
        // like: #.
        //       .#

        for candidate in current.iter(20) {
            let manhattan = current.manhattan(&candidate);
            if grid.in_grid(&candidate)
//                && grid[current + d] == b'#' don't need to check for this
                && visited[candidate].is_some()
            {
                if let Some(distance_previous) = visited[candidate] {
                    if distance - distance_previous - manhattan >= 100 {
                        count += 1;
                    }

                    // if distance - distance_previous - manhattan >= 50 {
                    //     *dbg_map
                    //         .entry(distance - distance_previous - manhattan)
                    //         .or_default() += 1;

                    //     println!(
                    //         "Shortcut at {:?} of length {}",
                    //         current,
                    //         distance - distance_previous - manhattan
                    //     );
                    // }
                }
            }
        }
    }

    // println!("{:?}", dbg_map);

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA_PATH: &str = "data/test/year2024/day20.txt";

    #[test]
    fn test_part_one() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_one(&input);

        assert_eq!(0, result);
    }

    #[test]
    fn test_part_two() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_two(&input);

        assert_eq!(0, result);
    }
}

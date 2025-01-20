use crate::{
    util::grid::Grid,
    util::point::{Point, CARDINALS, NEIGHBORS},
};

fn score_regions(grid: &Grid<u8>) -> usize {
    let mut visited: Grid<bool> = Grid::new_with_same_size(grid);

    let mut stack = Vec::new();

    let mut total = 0;

    // visit all points
    for x in 0..grid.size.x {
        for y in 0..grid.size.y {
            let current = Point::new(x, y);

            // already saw it
            if visited[current] {
                continue;
            }

            let current_char = grid[current];

            stack.push(Point::new(x, y));
            let mut size = 0;
            let mut perimeter = 0;

            // start search from this point
            while !stack.is_empty() {
                let p = stack.pop().unwrap();

                if visited[p] {
                    continue;
                } else {
                    size += 1;
                    visited[p] = true;

                    CARDINALS.iter().for_each(|dir| {
                        let next_point = p + *dir;

                        if grid.in_grid(&next_point) {
                            // unseen and part of the region
                            if !visited[next_point] && grid[next_point] == current_char {
                                // new point of the region to explore
                                stack.push(next_point);
                            } else if grid[next_point] != current_char {
                                perimeter += 1;
                            }
                        } else {
                            // off the board
                            perimeter += 1;
                        }
                    });
                }
            }

            total += size * perimeter;
        }
    }

    total
}

fn corner(pos: u8, left: u8, up: u8, up_left: u8) -> usize {
    let internal_corner = pos == left && pos == up && pos != up_left;
    let external_corner = pos != left && pos != up;

    if internal_corner || external_corner {
        1
    } else {
        0
    }
}

fn count_corners(grid: &Grid<u8>, pos: Point) -> usize {
    let c = grid[pos];
    let mut corners = 0;

    let neighbors = NEIGHBORS.map(|dir| grid.try_get(&(pos + dir)));

    // need to test four corners
    let corner_groups = [(0, 6, 7), (0, 2, 1), (2, 4, 3), (4, 6, 5)];

    for (left_idx, up_idx, up_left_idx) in corner_groups {
        let left = neighbors[left_idx].unwrap_or(&0);
        let up = neighbors[up_idx].unwrap_or(&0);
        let up_left = neighbors[up_left_idx].unwrap_or(&0);
        corners += corner(c, *left, *up, *up_left);
    }

    corners
}

fn dfs(grid: &Grid<u8>, visited: &mut Grid<bool>, stack: &mut Vec<Point>, pos: Point) -> usize {
    let current_char = grid[pos];

    stack.push(pos);

    let mut size = 0;
    let mut sides = 0;

    // start search from this point
    while let Some(p) = stack.pop() {
        if visited[p] {
            continue;
        } else {
            size += 1;
            sides += count_corners(grid, p);
            visited[p] = true;

            for dir in CARDINALS.iter() {
                let next_point = p + *dir;

                if let Some(next_char) = grid.try_get(&next_point) {
                    if *next_char == current_char {
                        stack.push(next_point);
                    }
                }
            }
        }
    }

    size * sides
}

fn score_regions_(grid: &Grid<u8>) -> usize {
    let mut visited: Grid<bool> = Grid::new_with_same_size(grid);

    let mut stack = Vec::new();

    let mut total = 0;

    // visit all points
    for pos in grid.iter() {
        if visited[pos] {
            continue;
        }

        total += dfs(grid, &mut visited, &mut stack, pos);
    }

    total
}

pub fn parse(input: &str) -> Grid<u8> {
    Grid::from_str(input)
}

pub fn solve_part_one(input: &Grid<u8>) -> usize {
    score_regions(input)
}

pub fn solve_part_two(input: &Grid<u8>) -> usize {
    score_regions_(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA_PATH: &str = "data/test/year2024/day12.txt";

    #[test]
    fn test_part_one() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_one(&input);

        assert_eq!(1930, result);
    }

    #[test]
    fn test_part_two() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_two(&input);

        assert_eq!(1206, result);
    }
}

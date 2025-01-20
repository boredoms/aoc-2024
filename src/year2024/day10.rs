use std::{collections::HashSet, mem::swap};

use crate::util::{
    grid::Grid,
    point::{Point, CARDINALS},
};

pub fn parse(input: &str) -> Grid<u8> {
    Grid::from_str(input)
}

pub fn score(grid: &Grid<u8>, p: Point) -> usize {
    let mut level = grid[p];
    let mut curr = HashSet::from([p]);
    let mut next = HashSet::new();

    while level < b'9' {
        for p in &curr {
            CARDINALS.map(|dir| {
                let q = *p + dir;
                if grid.in_grid(&q) && grid[q] == level + 1 {
                    next.insert(q);
                }
            });
        }
        //println!("{:?}", next);
        curr.clear();
        swap(&mut curr, &mut next);
        level += 1;
    }

    curr.len()
}

pub fn rate(grid: &Grid<u8>, p: Point) -> usize {
    // find all paths from p to reachable 9s
    let mut level = grid[p];
    let mut res = 0;

    let mut stack = vec![(p, level)];

    while !stack.is_empty() {
        let (q, l) = stack.pop().unwrap();

        if l == b'9' {
            res += 1;
            continue;
        }

        CARDINALS.map(|dir| {
            let r = q + dir;
            if grid.in_grid(&r) && grid[r] == l + 1 {
                stack.push((r, l + 1));
            }
        });
    }

    res
}

pub fn solve_part_one(grid: &Grid<u8>) -> usize {
    let mut res = 0;

    let starting_points = grid.find_all(|c| *c == b'0');

    for p in starting_points {
        let s = score(&grid, p);
        res += s;
    }

    res
}

pub fn solve_part_two(grid: &Grid<u8>) -> usize {
    let mut res = 0;

    let starting_points = grid.find_all(|c| *c == b'0');

    for p in starting_points {
        let s = rate(&grid, p);
        res += s;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA_PATH: &str = "data/test/year2024/day10.txt";

    #[test]
    fn test_part_one() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_one(&input);

        assert_eq!(36, result);
    }

    #[test]
    fn test_part_two() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_two(&input);

        assert_eq!(81, result);
    }
}

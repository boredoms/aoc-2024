use std::{collections::HashSet, mem::swap};

use crate::solvers::grid::Grid;
use crate::solvers::point::{Point, CARDINALS, UP};

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

pub fn solve_part_one(input: &str) -> usize {
    let grid = Grid::from_str(input);

    let mut res = 0;

    let starting_points = grid.find_all(|c| *c == b'0');

    for p in starting_points {
        let s = score(&grid, p);
        res += s;
    }

    res
}

pub fn solve_part_two(input: &str) -> usize {
    let grid = Grid::from_str(input);

    let mut res = 0;

    let starting_points = grid.find_all(|c| *c == b'0');

    for p in starting_points {
        let s = rate(&grid, p);
        println!("{:?}: {s}", p);
        res += s;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&std::fs::read_to_string("data/day10/input.txt").unwrap());
        assert_eq!(36, result);
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_two(&std::fs::read_to_string("data/day10/input.txt").unwrap());
        assert_eq!(81, result);
    }
}

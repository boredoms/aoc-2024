use std::mem::swap;

use crate::util::{
    grid::Grid,
    point::{Point, CARDINALS},
};

fn parse(input: &str, size: Point, num_fallen: usize) -> Grid<u8> {
    let mut grid = Grid::new_with_element(size.x, size.y, b'.');

    input.lines().take(num_fallen).for_each(|line| {
        let (a, b) = line.split_once(',').unwrap();

        grid[Point::new(a.parse().unwrap(), b.parse().unwrap())] = b'#';
    });

    grid
}

fn bfs(grid: &Grid<u8>) -> Option<usize> {
    let mut i = 0;

    let mut visited: Grid<bool> = Grid::new_with_same_size(grid);

    let mut curr = Vec::new();
    let mut next = Vec::new();

    let start = Point::new(0, 0);
    let goal = Point::new(grid.size.x - 1, grid.size.y - 1);

    curr.push(start);

    while !visited[goal] {
        if curr.is_empty() {
            return None;
        }

        while let Some(p) = curr.pop() {
            if visited[p] {
                continue;
            }

            visited[p] = true;

            for d in CARDINALS {
                if grid.in_grid(&(p + d)) && !visited[p + d] && grid[p + d] == b'.' {
                    next.push(p + d);
                }
            }
        }

        i += 1;
        swap(&mut curr, &mut next);
    }

    Some(i - 1)
}

pub fn solve_part_one(input: &str) -> usize {
    let grid = parse(input, Point::new(71, 71), 1024);
    bfs(&grid).unwrap()
}

pub fn solve_part_two(input: &str) -> usize {
    let mut grid = parse(input, Point::new(71, 71), 1024);

    for line in input.lines().skip(1024) {
        // place the tile
        let (a, b) = line.split_once(',').unwrap();
        let p = Point::new(a.parse().unwrap(), b.parse().unwrap());

        grid[p] = b'#';

        // check if it's still reachable
        if let None = bfs(&grid) {
            println!("{}", line);
            break;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&std::fs::read_to_string("data/day18/input.txt").unwrap());
        assert_eq!(22, result);
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_two(&std::fs::read_to_string("data/day18/input.txt").unwrap());
        assert_eq!(0, result);
    }
}

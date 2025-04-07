use std::{char, collections::HashSet};

use crate::util::{
    grid::{Grid, GridIterator},
    point::{Point, LEFT, RIGHT},
};

#[derive(Debug, Clone)]
pub struct Input {
    pos: Point,
    grid: Grid<u8>,
    moves: Vec<u8>,
}

pub fn parse<'a>(input: &'a str) -> &'a str {
    input
}

pub fn parse_(input: &str) -> Input {
    let (grid, moves) = input.split_once("\n\n").unwrap();

    let grid = Grid::from_str(grid);
    let pos = grid.find(|c| *c == b'@').unwrap();

    Input {
        pos,
        grid,
        moves: moves.bytes().filter(|c| *c != b'\n').collect(),
    }
}

fn expand(input: &str) -> Input {
    let (grid, moves) = input.split_once("\n\n").unwrap();

    let wider: String = grid
        .chars()
        .map(|c| match c {
            '#' => "##".to_string(),
            'O' => "[]".to_string(),
            '.' => "..".to_string(),
            '@' => "@.".to_string(),
            x => x.to_string(),
        })
        .collect();

    let grid = Grid::from_str(&wider);
    let pos = grid.find(|c| *c == b'@').unwrap();

    Input {
        pos,
        grid,
        moves: moves.bytes().filter(|c| *c != b'\n').collect(),
    }
}

fn attempt_move(grid: &mut Grid<u8>, pos: &mut Point, dir: Point) {
    let mut new_pos = *pos + dir;
    let mut free_spaces = 0;

    // count the number of free spaces to move
    while grid[new_pos] != b'#' {
        if grid[new_pos] == b'.' {
            free_spaces = 1;
            break;
        }

        new_pos += dir;
    }

    if free_spaces == 0 {
        return;
    } else {
        grid[*pos] = b'.';
        grid[*pos + dir] = b'@';

        *pos += dir;

        while new_pos != *pos {
            grid[new_pos] = b'O';
            new_pos -= dir;
        }
    }
}

fn collect_coordinates(grid: &Grid<u8>, pos: Point, dir: Point) -> Option<Vec<Point>> {
    let mut queue = vec![pos];
    let mut res = Vec::new();
    let mut seen = Grid::new_with_same_size(grid);

    while !queue.is_empty() {
        let p = queue.pop().unwrap();

        if seen[p] {
            continue;
        }

        seen[p] = true;

        match grid[p] {
            b'#' => return None,
            b'.' => continue,
            b'@' => {
                queue.push(p + dir);
                res.push(p);
            }
            b'[' => {
                if !seen[p + RIGHT] {
                    queue.push(p + RIGHT);
                }
                queue.push(p + dir);
                res.push(p)
            }
            b']' => {
                if !seen[p + LEFT] {
                    queue.push(p + LEFT);
                }
                queue.push(p + dir);
                res.push(p);
            }
            _ => panic!("unexpected char in grid"),
        }
    }

    Some(res)
}

fn attempt_move_wide(grid: &mut Grid<u8>, pos: &mut Point, dir: Point) {
    // in the case we move left or right
    if dir == RIGHT || dir == LEFT {
        let mut possible = false;
        let mut new_pos = *pos + dir;

        while grid[new_pos] != b'#' {
            if grid[new_pos] == b'.' {
                possible = true;
                break;
            }
            new_pos += dir;
        }

        if possible {
            while new_pos != *pos {
                grid[new_pos] = grid[new_pos - dir];
                new_pos -= dir;
            }

            grid[*pos] = b'.';
            *pos += dir;
        }
    } else {
        if let Some(mut coordinates) = collect_coordinates(grid, *pos, dir) {
            // sort coordinates
            coordinates.sort_by_key(|p| -dir.y * p.y);

            //println!("{:?}", coordinates);

            for p in coordinates {
                grid[p + dir] = grid[p];
                grid[p] = b'.';
            }
            *pos += dir;
        }
    }
}

fn score(grid: Grid<u8>) -> usize {
    grid.find_all(|c| *c == b'O' || *c == b'[')
        .iter()
        .map(|p| p.x + 100 * p.y)
        .sum::<i64>() as usize
}

pub fn solve_part_one(input: &str) -> usize {
    let mut input = parse_(input);

    input
        .moves
        .iter()
        .for_each(|c| attempt_move(&mut input.grid, &mut input.pos, Point::from_u8(*c)));

    score(input.grid)
}

pub fn solve_part_two(input: &str) -> usize {
    let mut input = expand(input);

    for ele in input.moves.iter() {
        attempt_move_wide(&mut input.grid, &mut input.pos, Point::from_u8(*ele));
    }

    score(input.grid)
}

pub fn solve(filename: &str) -> Result<(String, String), String> {
    let input =
        &std::fs::read_to_string(filename).or(Err(format!("could not read file {}", filename)))?;

    let input = parse(input);

    Ok((
        solve_part_one(&input).to_string(),
        solve_part_two(&input).to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA_PATH: &str = "data/test/year2024/day15.txt";

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

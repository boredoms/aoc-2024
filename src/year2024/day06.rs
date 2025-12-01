use std::{collections::HashSet, hash::Hash, str::FromStr};

use crate::util::{
    grid::Grid,
    point::{Point, DOWN, LEFT, RIGHT, UP},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Guard {
    Up,
    Right,
    Left,
    Down,
}

impl Guard {
    fn to_u8(&self) -> u8 {
        match self {
            Guard::Up => 0b1000,
            Guard::Right => 0b0100,
            Guard::Down => 0b0010,
            Guard::Left => 0b0001,
        }
    }

    fn to_tuple(&self) -> (i32, i32) {
        match self {
            Guard::Up => (0, -1),
            Guard::Down => (0, 1),
            Guard::Left => (-1, 0),
            Guard::Right => (1, 0),
        }
    }

    fn rotate(&self) -> Guard {
        match self {
            Guard::Up => Guard::Right,
            Guard::Right => Guard::Down,
            Guard::Down => Guard::Left,
            Guard::Left => Guard::Up,
        }
    }
}

impl FromStr for Guard {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "^" => Ok(Guard::Up),
            ">" => Ok(Guard::Right),
            "<" => Ok(Guard::Left),
            "v" => Ok(Guard::Down),
            _ => Err(String::from("Could not parse Guard!")),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum CellState {
    Empty,
    Blocked,
}

impl ToString for CellState {
    fn to_string(&self) -> String {
        match self {
            CellState::Empty => ".".to_string(),
            CellState::Blocked => "#".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    grid: Vec<CellState>,
    grid_size: (i32, i32),
    starting_position: (i32, i32),
    guard_position: (i32, i32),
    guard_direction: Guard,
    can_block: usize,
    seen: HashSet<(i32, i32)>,
    visited: HashSet<((i32, i32), Guard)>,
    lookahead: HashSet<((i32, i32), Guard)>,
    // might want to take care of loops here
}

impl Map {
    fn move_guard(&mut self) -> Result<(), String> {
        let (x, y) = self.guard_direction.to_tuple();
        let (guard_x, guard_y) = self.guard_position;

        let (new_x, new_y) = (guard_x + x, guard_y + y);

        if !valid_position(&(new_x, new_y), &self.grid_size) {
            // can't block because box would go outside grid

            self.visited
                .insert((self.guard_position, self.guard_direction));

            return Err("Guard is out of bounds".to_string());
        }

        let i = (new_y * self.grid_size.0 + new_x) as usize;

        match self.grid[i] {
            CellState::Empty => {
                // if it is empty, we can move into it

                // add current position as visited
                self.visited
                    .insert((self.guard_position, self.guard_direction));
                self.seen.insert(self.guard_position);

                // then try rotating and see what happens
                if self.starting_position != (new_x, new_y) && !self.seen.contains(&(new_x, new_y))
                {
                    self.grid[i] = CellState::Blocked;

                    self.can_block +=
                        self.find_loop(self.guard_position, self.guard_direction.rotate());

                    self.grid[i] = CellState::Empty;
                }

                // then move into it
                self.guard_position = (new_x, new_y);
            }
            CellState::Blocked => {
                self.guard_direction = self.guard_direction.rotate();
                return self.move_guard();
            }
        }

        Ok(())
    }

    fn print_grid(&self) {
        for i in 0..self.grid_size.1 {
            for j in 0..self.grid_size.0 {
                print!(
                    "{:4},",
                    self.grid[(i * self.grid_size.0 + j) as usize].to_string()
                );
            }
        }
    }

    fn find_loop(&mut self, mut pos: (i32, i32), mut dir: Guard) -> usize {
        loop {
            let (x, y) = dir.to_tuple();
            let new_pos = (pos.0 + x, pos.1 + y);

            // check if new position is valid, if not, we left the grid and so no loop happens
            if !valid_position(&new_pos, &self.grid_size) {
                self.lookahead.clear();
                return 0;
            }

            let i = new_pos.1 * self.grid_size.0 + new_pos.0;

            match self.grid[i as usize] {
                CellState::Empty => {
                    if self.visited.contains(&(pos, dir)) || self.lookahead.contains(&(pos, dir)) {
                        self.lookahead.clear();
                        return 1;
                    }

                    self.lookahead.insert((pos, dir));

                    pos = new_pos;
                }
                CellState::Blocked => {
                    dir = dir.rotate();
                    continue;
                }
            }
        }
    }
}

fn valid_position(pos: &(i32, i32), size: &(i32, i32)) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < size.0 && pos.1 < size.1
}

pub fn parse(input: &str) -> Map {
    let grid_size = (
        input.lines().next().unwrap().len() as i32,
        input.lines().count() as i32,
    );

    let grid: Vec<CellState> = input
        .as_bytes()
        .iter()
        .filter(|c| **c != b'\n')
        .map(|c| match c {
            b'.' => CellState::Empty,
            b'^' => CellState::Empty,
            b'#' => CellState::Blocked,
            _ => panic!("unexpected char"),
        })
        .collect();

    let i = input
        .as_bytes()
        .iter()
        .filter(|c| **c != b'\n')
        .position(|c| *c == b'^')
        .unwrap() as i32;

    let guard_position = (i % grid_size.0, i / grid_size.0);
    let starting_position = guard_position;

    Map {
        visited: HashSet::with_capacity(grid.len() * 4),
        lookahead: HashSet::with_capacity(grid.len() * 4),
        seen: HashSet::with_capacity(grid.len()),
        grid,
        grid_size,
        starting_position,
        guard_position,
        guard_direction: Guard::Up,
        can_block: 0,
    }
}

pub fn solve_part_one(input: &Map) -> usize {
    let mut map = input.clone();

    while map.move_guard().is_ok() {}

    let mut unique = HashSet::with_capacity(map.grid.len());

    for (pos, _) in map.visited.iter() {
        unique.insert(*pos);
    }

    unique.len()
}

pub struct Input {
    map: Grid<u8>,
    start: Point,
}

pub fn parse_(input: &str) -> Input {
    let map = Grid::from_str(input);
    let start = map.find(|c| *c == b'^').expect("map must have a guard");

    Input { map, start }
}

fn rotate_clockwise(direction: &Point) -> Point {
    match *direction {
        UP => RIGHT,
        RIGHT => DOWN,
        DOWN => LEFT,
        LEFT => UP,
        _ => unreachable!("this only works on cardinal directions"),
    }
}

pub fn solve_part_one_(input: &Input) -> usize {
    let mut direction = UP;
    let mut position = input.start;
    let mut seen = Grid::new_with_same_size(&input.map);

    seen[position] = true;
    let mut res = 1;

    while let Some(p) = input.map.try_get(&(position + direction)) {
        match p {
            b'^' | b'.' => {
                position += direction;

                if !seen[position] {
                    res += 1;
                    seen[position] = true;
                }
            }
            b'#' => direction = rotate_clockwise(&direction),
            _ => unreachable!(),
        }
    }

    res
}

pub fn solve_part_two_(input: &Input) -> usize {
    0
}

pub fn solve_part_two(input: &Map) -> usize {
    let mut map = input.clone();

    while map.move_guard().is_ok() {}

    // map.print_grid();

    map.can_block
}

pub fn solve(filename: &str) -> Result<(String, String), String> {
    let input =
        &std::fs::read_to_string(filename).or(Err(format!("could not read file {}", filename)))?;

    let uwu = parse_(input);

    let input = parse(input);

    Ok((
        solve_part_one_(&uwu).to_string(),
        solve_part_two(&input).to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA_PATH: &str = "data/test/year2024/day06.txt";

    #[test]
    fn test_part_one() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_one(&input);

        assert_eq!(41, result);
    }

    #[test]
    fn test_part_two() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_two(&input);

        assert_eq!(6, result);
    }
}

use crate::util::{grid::Grid, point::Point};

#[derive(Debug)]
struct Input {
    pos: Point,
    grid: Grid<u8>,
    moves: Vec<u8>,
}

fn parse(input: &str) -> Input {
    let (grid, moves) = input.split_once("\n\n").unwrap();

    let grid = Grid::from_str(grid);
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

fn score(grid: Grid<u8>) -> usize {
    grid.find_all(|c| *c == b'O')
        .iter()
        .map(|p| p.x + 100 * p.y)
        .sum::<i64>() as usize
}

pub fn solve_part_one(input: &str) -> usize {
    let mut input = parse(input);

    input
        .moves
        .iter()
        .for_each(|c| attempt_move(&mut input.grid, &mut input.pos, Point::from_u8(*c)));

    score(input.grid)
}

pub fn solve_part_two(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&std::fs::read_to_string("data/day15/input.txt").unwrap());
        assert_eq!(0, result);
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_two("");
        assert_eq!(0, result);
    }
}

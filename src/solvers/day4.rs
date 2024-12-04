struct Grid {
    chars: Vec<char>,
    size: Index,
}

impl Grid {
    fn get(&self, index: &Index) -> char {
        self.chars[(index.y * self.size.x + index.x) as usize]
    }

    fn get_safe(&self, index: &Index) -> Option<&char> {
        self.chars.get((index.y * self.size.x + index.x) as usize)
    }

    fn num_chars(&self) -> usize {
        self.chars.len()
    }

    fn to_index(&self, i: i32) -> Index {
        Index {
            x: i % self.size.x,
            y: i / self.size.x,
        }
    }
}

struct Index {
    x: i32,
    y: i32,
}

enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

static DIRECTIONS: [Direction; 8] = [
    Direction::N,
    Direction::NE,
    Direction::E,
    Direction::SE,
    Direction::S,
    Direction::SW,
    Direction::W,
    Direction::NW,
];

impl Direction {
    fn to_tuple(&self) -> (i32, i32) {
        match self {
            Direction::N => (0, -1),
            Direction::NE => (1, -1),
            Direction::E => (1, 0),
            Direction::SE => (1, 1),
            Direction::S => (0, 1),
            Direction::SW => (-1, 1),
            Direction::W => (-1, 0),
            Direction::NW => (-1, -1),
        }
    }
}

fn parse(input: &str) -> Grid {
    let y = input.lines().count() as i32;
    let x = input.split_once('\n').unwrap().0.len() as i32;
    let chars: Vec<char> = input.lines().map(|line| line.chars()).flatten().collect();

    Grid {
        chars,
        size: Index { x, y },
    }
}

fn take_step(grid: &Grid, index: &Index, d: &Direction) -> Option<Index> {
    let (i, j) = d.to_tuple();
    let (x, y) = (index.x + i, index.y + j);

    if x < 0 || x >= grid.size.x || y < 0 || y >= grid.size.y {
        None
    } else {
        Some(Index { x, y })
    }
}

fn grid_search_direction(grid: &Grid, index: &Index, d: &Direction, needle: &Vec<char>) -> usize {
    let mut i = 1;
    let mut pos = Index {
        x: index.x,
        y: index.y,
    };

    while let Some(index) = take_step(grid, &pos, d) {
        if grid.get(&index) != needle[i] {
            return 0;
        }
        i += 1;
        pos = index;

        if i == needle.len() {
            return 1;
        }
    }

    0
}

fn grid_search(grid: &Grid, index: &Index, needle: &Vec<char>) -> usize {
    if grid.get(index) != needle[0] {
        0
    } else {
        DIRECTIONS
            .iter()
            .map(|d| grid_search_direction(grid, index, d, needle))
            .sum()
    }
}

pub fn solve_part_one(input: &str) -> usize {
    let grid = parse(input);
    let needle = vec!['X', 'M', 'A', 'S'];

    (0..grid.num_chars())
        .map(|i| grid_search(&grid, &grid.to_index(i as i32), &needle))
        .sum()
}

fn x_search(grid: &Grid, index: &Index) -> usize {
    if grid.get(index) != 'A' {
        return 0;
    }

    if let [Some(a), Some(b), Some(c), Some(d)] =
        [Direction::NW, Direction::NE, Direction::SE, Direction::SW].map(|d| {
            take_step(grid, index, &d)
                .map(|f| grid.get_safe(&f))
                .flatten()
        })
    {
        match (a, b, c, d) {
            ('M', 'M', 'S', 'S') => return 1,
            ('S', 'M', 'M', 'S') => return 1,
            ('S', 'S', 'M', 'M') => return 1,
            ('M', 'S', 'S', 'M') => return 1,
            _ => return 0,
        }
    }

    0
}

pub fn solve_part_two(input: &str) -> usize {
    let grid = parse(input);

    (0..grid.num_chars())
        .map(|i| x_search(&grid, &grid.to_index(i as i32)))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&std::fs::read_to_string("data/day4/test.txt").unwrap());
        // let result = solve_part_one("XMAS\n");
        assert_eq!(18, result);
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_two(&std::fs::read_to_string("data/day4/input.txt").unwrap());
        assert_eq!(9, result);
    }
}
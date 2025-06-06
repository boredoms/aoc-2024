use std::{collections::VecDeque, mem::swap};

use crate::util::{
    grid::Grid,
    point::{Point, CARDINALS, RIGHT},
};

pub fn parse(input: &str) -> Grid<u8> {
    Grid::from_str(input)
}

pub fn solve_part_one(input: &Grid<u8>) -> usize {
    let start = Point::new(1, input.size.y - 2);
    let end = Point::new(input.size.x - 2, 1);
    let starting_direction = RIGHT;

    let mut visited = Grid::new_with_same_size(&input);
    visited[start] = true;

    let mut queue: Vec<Vec<(Point, Point)>> = vec![Vec::new(); 1001];
    queue[0] = vec![(start, starting_direction)];

    let mut i = 0;
    let mut cost = 0;

    loop {
        let mut current = Vec::new();
        swap(&mut current, &mut queue[i]);

        for (pos, dir) in current {
            // by the design of our algorithm, we have the invariant that if we process a point here, it will be at the shortest distance from the start
            if pos == end {
                return cost;
            }

            visited[pos] = true;

            for d in CARDINALS {
                let candidate = pos + d;
                if input.in_grid(&candidate)
                    && (input[candidate] == b'.' || input[candidate] == b'E')
                    && !visited[candidate]
                {
                    // valid step to take
                    if d == dir {
                        queue[(i + 1) % 1001].push((candidate, d));
                    } else {
                        queue[(i + 1001) % 1001].push((candidate, d));
                    }
                }
            }
        }

        i += 1;
        cost += 1;
        i %= 1001;
    }
}

pub fn solve_part_two(input: &Grid<u8>) -> usize {
    let start = Point::new(1, input.size.y - 2);
    let end = Point::new(input.size.x - 2, 1);
    let starting_direction = RIGHT;

    let mut visited = Grid::new_with_same_size(input);
    visited[start] = Some((starting_direction, 0));

    let mut queue: Vec<Vec<(Point, Point)>> = vec![Vec::new(); 1001];
    queue[0] = vec![(start, starting_direction)];

    let mut i = 0;
    let mut cost = 0;

    let mut done = false;

    while !done {
        let mut current = Vec::new();
        swap(&mut current, &mut queue[i]);

        for (pos, dir) in current {
            // by the design of our algorithm, we have the invariant that if we process a point here, it will be at the shortest distance from the start
            if let None = visited[pos] {
                visited[pos] = Some((dir, cost));
            }

            if pos == end {
                visited.print();
                done = true;
                break;
            }

            for d in CARDINALS {
                let candidate = pos + d;
                if input.in_grid(&candidate)
                    && (input[candidate] == b'.' || input[candidate] == b'E')
                    && visited[candidate].is_none()
                {
                    // valid step to take
                    if d == dir {
                        queue[(i + 1) % 1001].push((candidate, d));
                    } else {
                        queue[(i + 1001) % 1001].push((candidate, d));
                    }
                }
            }
        }

        i += 1;
        cost += 1;
        i %= 1001;
    }

    // search backward
    let mut plot = input.clone();
    plot[end] = b'O';

    let mut queue = VecDeque::new();
    queue.push_back(end);

    let mut count = 1;

    while !queue.is_empty() {
        let p = queue.pop_front().unwrap();

        let (dd, cost) = visited[p].unwrap();

        // check the neighbors
        for d in CARDINALS {
            let candidate = p + d;

            // if it is on a shortest path it needs to be
            // a valid field
            if input.in_grid(&candidate)
                && (input[candidate] == b'.' || input[candidate] == b'S')
                && visited[candidate].is_some()
                && plot[candidate] != b'O'
            {
                let (dir, c) = visited[candidate].unwrap();
                // it also must be possible to get from this field to the current one with the right cost difference
                if (dir == dd && cost == c + 1) // by moving straight forward
                    || (dir != dd && cost == c + 1001) // if we took a turn at this
                    || (dir != dd && cost == c - 999 && plot[candidate + dir + dir] == b'O')
                // when two paths join
                {
                    count += 1;
                    queue.push_back(candidate);

                    plot[candidate] = b'O';
                }
            }
        }
    }

    println!("{:?}", visited[Point::new(1, 10)]);
    println!("{:?}", visited[Point::new(3, 10)]);
    println!("{:?}", visited[Point::new(4, 7)]);
    println!("{:?}", visited[Point::new(5, 7)]);

    plot.print();

    count
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

    static TEST_DATA_PATH: &str = "data/test/year2024/day16.txt";

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

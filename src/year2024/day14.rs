use regex::Regex;

use crate::util::{grid::Grid, point::Point};

#[derive(Debug)]
pub struct Robot {
    p: Point,
    v: Point,
}

type Input = Vec<Robot>;

pub fn parse(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"p=(\d+),(\d+) v=(\-*\d+),(\-*\d+)").unwrap();
    let mut res = Vec::new();

    input.lines().for_each(|line| {
        let caps = re
            .captures(line)
            .expect("Expected valid robot specification");

        res.push(Robot {
            p: Point::new(caps[1].parse().unwrap(), caps[2].parse().unwrap()),
            v: Point::new(caps[3].parse().unwrap(), caps[4].parse().unwrap()),
        })
    });

    res
}

fn move_robot(robot: &Robot, room_size: &Point, seconds: i64) -> Point {
    let mut p = (robot.p + robot.v * seconds) % *room_size;

    if p.x < 0 {
        p.x += room_size.x;
    }

    if p.y < 0 {
        p.y += room_size.y;
    }

    p
}

// return the quadrant
// +---+
// |0|1|
// |-+-|
// |2|3|
// +---+
fn quadrant(pos: Point, room_size: &Point) -> Option<usize> {
    let half_x = room_size.x / 2;
    let half_y = room_size.y / 2;

    if pos.x >= 0 && pos.x < half_x && pos.y >= 0 && pos.y < half_y {
        return Some(0);
    }

    if pos.x > half_x && pos.x < room_size.x && pos.y >= 0 && pos.y < half_y {
        return Some(1);
    }

    if pos.x >= 0 && pos.x < half_x && pos.y > half_y && pos.y < room_size.y {
        return Some(2);
    }

    // println!("pososos {:?}", pos);

    if pos.x > half_x && pos.x < room_size.x && pos.y > half_y && pos.y < room_size.y {
        return Some(3);
    }

    None
}

pub fn solve_part_one(input: &Input) -> usize {
    let seconds = 100;
    let room_size = Point::new(101, 103);

    // println!("{:?}", robots);

    let mut quadrant_counts = vec![0; 4];

    input
        .iter()
        .map(|robot| move_robot(robot, &room_size, seconds))
        // .inspect(|p| println!("{:?}", p))
        .filter_map(|p| quadrant(p, &room_size))
        .for_each(|q| quadrant_counts[q] += 1);

    // println!("{:?}", quadrant_counts);

    quadrant_counts.iter().product()
}

fn print_robots(robots: &Vec<Point>) {
    let mut grid = Grid::new_with_element(101, 103, '.');

    for robot in robots {
        grid[*robot] = '#';
    }

    grid.print();
}

fn scan_clusters(points: Vec<Point>, room_size: &Point) -> bool {
    let mut x_counts = vec![0; room_size.x as usize];
    let mut y_counts = vec![0; room_size.y as usize];

    let c = 20;
    let d = 15;

    for p in points {
        x_counts[p.x as usize] += 1;
        y_counts[p.y as usize] += 1;
    }

    if x_counts.iter().filter(|i| **i <= 3).count() > (((3 * room_size.x) / 5) as usize)
        && y_counts.iter().filter(|i| **i <= 3).count() > (((3 * room_size.y) / 5) as usize)
    {
        return true;
    }

    false
}

pub fn solve_part_two(input: &Input) -> usize {
    let mut i = 0;
    let room_size = Point::new(101, 103);

    while !scan_clusters(
        input
            .iter()
            .map(|robot| move_robot(robot, &room_size, i))
            .collect(),
        &room_size,
    ) {
        i += 1;
    }

    i as usize
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

    static TEST_DATA_PATH: &str = "data/test/year2024/day14.txt";

    #[test]
    fn test_part_one() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_one(&input);

        assert_eq!(12, result);
    }

    #[test]
    fn test_part_two() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_two(&input);

        assert_eq!(0, result);
    }
}

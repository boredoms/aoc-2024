use std::collections::{HashMap, HashSet};

use crate::util::grid::Grid;

pub fn parse(input: &str) -> Grid<u8> {
    Grid::from_str(input)
}

pub fn solve_part_one(input: &Grid<u8>) -> usize {
    let mut unique = HashSet::new();

    input.elements.iter().for_each(|c| {
        if *c != b'.' {
            unique.insert(c);
        }
    });

    let mut grouped = HashMap::new();

    unique.iter().for_each(|c| {
        grouped.insert(**c, input.find_all(|x| *x == **c));
    });

    let mut antinodes = HashSet::new();

    for (_, v) in grouped.iter() {
        for p in v {
            for q in v {
                if p == q {
                    continue;
                }

                let diff = *p - *q;

                let antinode = *q - diff;
                if input.in_grid(&antinode) {
                    antinodes.insert(antinode);
                }
                let antinode = *p + diff;
                if input.in_grid(&antinode) {
                    antinodes.insert(antinode);
                }
            }
        }
    }

    antinodes.len()
}

pub fn solve_part_two(input: &Grid<u8>) -> usize {
    let mut unique = HashSet::new();

    input.elements.iter().for_each(|c| {
        if *c != b'.' {
            unique.insert(c);
        }
    });

    let mut grouped = HashMap::new();

    unique.iter().for_each(|c| {
        grouped.insert(**c, input.find_all(|x| *x == **c));
    });

    let mut antinodes = HashSet::new();

    for (_, v) in grouped.iter() {
        for p in v {
            for q in v {
                if p == q {
                    continue;
                }

                let diff = *p - *q;

                // instead of calculating the antinode once, we do it until fail
                let mut antinode = *p;

                while input.in_grid(&antinode) {
                    antinodes.insert(antinode);
                    antinode = antinode + diff;
                }

                let mut antinode = *q;
                while input.in_grid(&antinode) {
                    antinodes.insert(antinode);
                    antinode = antinode - diff;
                }
            }
        }
    }

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA_PATH: &str = "data/test/year2024/day08.txt";

    #[test]
    fn test_part_one() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_one(&input);

        assert_eq!(14, result);
    }

    #[test]
    fn test_part_two() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_two(&input);

        assert_eq!(34, result);
    }
}

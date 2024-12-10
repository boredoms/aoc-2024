use std::collections::{hash_set, HashMap, HashSet};

use crate::solvers::grid::Grid;

pub fn solve_part_one(input: &str) -> usize {
    let input = Grid::from_str(input);

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

pub fn solve_part_two(input: &str) -> usize {
    let input = Grid::from_str(input);

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

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&std::fs::read_to_string("data/day8/input.txt").unwrap());
        assert_eq!(14, result);
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_two(&std::fs::read_to_string("data/day8/input.txt").unwrap());
        assert_eq!(34, result);
    }
}

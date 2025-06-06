use std::{cmp, collections::HashMap};

type Ordering = HashMap<(u32, u32), cmp::Ordering>;

#[derive(Debug, Clone)]
pub struct Input {
    pages: Vec<Vec<u32>>,
    ordering: Ordering,
}

pub fn parse(input: &str) -> Input {
    let (rules, pages) = input.split_once("\n\n").unwrap();

    let mut ordering = HashMap::new();

    rules.lines().for_each(|s| {
        let (a, b) = s.split_once('|').unwrap();
        let a = a.parse().unwrap();
        let b = b.parse().unwrap();

        if a < b {
            ordering.insert((a, b), cmp::Ordering::Less);
        } else {
            ordering.insert((b, a), cmp::Ordering::Greater);
        }
    });

    Input {
        pages: pages
            .lines()
            .map(|s| s.split(',').map(|a| a.parse().unwrap()).collect())
            .collect(),
        ordering,
    }
}

fn middle<T>(list: &[T]) -> &T {
    list.get(list.len() / 2).unwrap()
}

fn is_ordered(pages: &[u32], ordering: &Ordering) -> bool {
    pages.is_sorted_by(|a, b| {
        if a < b {
            *ordering.get(&(*a, *b)).unwrap() == cmp::Ordering::Less
        } else {
            *ordering.get(&(*b, *a)).unwrap() == cmp::Ordering::Greater
        }
    })
}

pub fn solve_part_one(input: &Input) -> usize {
    input
        .pages
        .iter()
        .filter(|pages| is_ordered(pages, &input.ordering))
        .map(|p| middle(p))
        .sum::<u32>() as usize
}

pub fn solve_part_two(input: &Input) -> usize {
    input
        .pages
        .clone()
        .iter_mut()
        .filter(|pages| !is_ordered(pages, &input.ordering))
        .map(|pages| {
            pages.sort_by(|a, b| {
                if a < b {
                    *input.ordering.get(&(*a, *b)).unwrap()
                } else {
                    input.ordering.get(&(*b, *a)).unwrap().reverse()
                }
            });

            middle(pages)
        })
        .sum::<u32>() as usize
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

    static TEST_DATA_PATH: &str = "data/test/year2024/day05.txt";

    #[test]
    fn test_part_one() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_one(&input);

        assert_eq!(143, result);
    }

    #[test]
    fn test_part_two() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_two(&input);

        assert_eq!(123, result);
    }
}

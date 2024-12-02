use std::path::Iter;

struct Input {
    lists: Vec<Vec<i32>>,
}

fn parse(input: &str) -> Input {
    let lists = input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|n| n.parse().expect("could not parse the input"))
                .collect()
        })
        .collect();

    Input { lists }
}

fn is_safe(list: &Vec<i32>) -> bool {
    let mut diffs = list.windows(2).map(|w| w[0] - w[1]).collect::<Vec<_>>();

    if diffs[0].is_negative() {
        diffs.iter_mut().for_each(|i| *i = -*i);
    }

    diffs.iter().all(|&i| i < 4 && i > 0)
}

fn skip_iter<'a, T>(s: &'a [T], n: usize) -> impl Iterator<Item = &T> {
    s.iter().take(n).chain(s.iter().skip(n + 1)).into_iter()
}

fn generate_sublists(list: &Vec<i32>) -> impl Iterator<Item = Vec<i32>> + '_ {
    (0..list.len()).map(|i| {
        let mut new_list = list.to_vec();
        new_list.remove(i);
        new_list
    })
}

pub fn solve_part_one(input: &str) -> usize {
    let input = parse(input);

    input.lists.into_iter().filter(|list| is_safe(list)).count()
}

pub fn solve_part_two(input: &str) -> usize {
    // solution goes here
    let input = parse(input);

    input
        .lists
        .into_iter()
        .filter(|list| generate_sublists(list).any(|l| is_safe(&l)))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&std::fs::read_to_string("data/day2/test.txt").unwrap());
        assert_eq!(2, result);
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_two(&std::fs::read_to_string("data/day2/test.txt").unwrap());
        assert_eq!(4, result);
    }
}

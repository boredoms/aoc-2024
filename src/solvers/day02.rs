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

fn is_safe(list: &[i32]) -> bool {
    list.windows(2)
        .map(|w| w[0] - w[1])
        .all(|d| d.abs() > 0 && d.abs() < 4)
        && (list.iter().is_sorted_by(|a, b| a < b) || list.iter().is_sorted_by(|a, b| a > b))
}

fn generate_sublists(list: &[i32]) -> impl Iterator<Item = Vec<i32>> + '_ {
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

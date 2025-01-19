pub struct Input {
    lists: Vec<Vec<i32>>,
}

pub fn parse(input: &str) -> Input {
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

pub fn solve_part_one(input: &Input) -> usize {
    input.lists.iter().filter(|list| is_safe(list)).count()
}

pub fn solve_part_two(input: &Input) -> usize {
    input
        .lists
        .iter()
        .filter(|list| generate_sublists(list).any(|l| is_safe(&l)))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA_PATH: &str = "data/test/year2024/day02.txt";

    #[test]
    fn test_part_one() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_one(&input);

        assert_eq!(2, result);
    }

    #[test]
    fn test_part_two() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_two(&input);

        assert_eq!(4, result);
    }
}

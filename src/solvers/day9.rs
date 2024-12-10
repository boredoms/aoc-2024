fn expand(input: &str) -> Vec<Option<u32>> {
    let len: usize = input.chars().fold(0, |sum, x| {
        sum + (x.to_digit(10).unwrap_or_default() as usize)
    });

    let mut expanded = Vec::with_capacity(len);
    let mut block_id = 0;

    for (i, c) in input.chars().enumerate() {
        let c = c.to_digit(10u32).unwrap_or_default();

        for _ in 0..c {
            if i % 2 == 0 {
                expanded.push(Some(block_id));
            } else {
                expanded.push(None);
            }
        }
        if i % 2 == 0 {
            block_id += 1;
        }
    }

    expanded
}

pub fn solve_part_one(input: &str) -> usize {
    let mut expanded = expand(input);

    // defragment
    let mut head: usize = 0;
    let mut tail = expanded.len() - 1;

    // defragment
    loop {
        // advance to next free spot
        while let Some(_) = expanded[head] {
            head += 1;
        }

        // find first filled spot
        while let None = expanded[tail] {
            tail -= 1;
        }

        // if we advanced head past tail, we are done
        if head >= tail {
            break;
        }

        expanded.swap(head, tail);
    }

    // calculate checksum
    let mut checksum = 0;

    for e in expanded.iter().enumerate() {
        match e {
            (i, Some(x)) => checksum += i * (*x as usize),
            (_, None) => break,
        }
    }

    checksum
}

pub fn solve_part_two(input: &str) -> usize {
    let mut expanded = expand(input);

    // defragment
    let mut head: usize = 0;
    let mut gap_size = 0;
    let mut file_size = 0;
    let mut tail = expanded.len() - 1;

    // defragment
    loop {
        // find first filled spot
        while let None = expanded[tail] {
            tail -= 1;
        }

        // and its size
        let file_type = expanded[tail].unwrap();

        // first file reached
        if file_type == 0 {
            break;
        }

        file_size = 0;
        while let Some(i) = expanded[tail - file_size] {
            if i != file_type {
                break;
            }
            file_size += 1;
        }

        // look for a gap
        loop {
            while let Some(_) = expanded[head] {
                head += 1;
            }

            if head >= tail {
                break;
            }

            gap_size = 0;
            while let None = expanded[head + gap_size] {
                gap_size += 1;
            }

            // if it fits, switch
            if gap_size >= file_size {
                for i in 0..file_size {
                    expanded.swap(head + i, tail - i);
                }
                break;
            } else {
                // otherwise advance
                head += gap_size;
            }
        }
        tail -= file_size;
        head = 0;

        //println!("{:?}", expanded);
    }

    //println!("{:?}", expanded);

    // calculate checksum
    let mut checksum = 0;

    for e in expanded.iter().enumerate() {
        match e {
            (i, Some(x)) => checksum += i * (*x as usize),
            (_, None) => (),
        }
    }

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&std::fs::read_to_string("data/day9/input.txt").unwrap());
        assert_eq!(1928, result);
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_two(&std::fs::read_to_string("data/day9/input.txt").unwrap());
        assert_eq!(2858, result);
    }
}

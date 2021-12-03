use common::{init, read_input_lines_usize};

fn part1(input: &[usize]) -> usize {
    input
        .iter()
        .fold((0, None), |(count, previous), current| match previous {
            Some(previous) if current > previous => (count + 1, Some(current)),
            Some(_) => (count, Some(current)),
            None => (count, Some(current)),
        })
        .0
}

fn part2(input: &[usize]) -> usize {
    input
        .iter()
        .as_slice()
        .windows(3)
        .map(|values| values.iter().sum())
        .fold(
            (0, None),
            |(count, previous): (usize, Option<usize>), current| match previous {
                Some(previous) if current > previous => (count + 1, Some(current)),
                Some(_) => (count, Some(current)),
                None => (count, Some(current)),
            },
        )
        .0
}

fn main() {
    let opt = init();
    let input = read_input_lines_usize(&opt.infile);
    if opt.part == 1 {
        let result_part1 = part1(&input);
        println!("Part 1: {}", result_part1);
    } else if opt.part == 2 {
        let result_part2 = part2(&input);
        println!("Part 2: {}", result_part2);
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    use std::path::PathBuf;

    const TEST_INPUT_FILE: &str = "inputs/input_test";

    #[test]
    fn test_part_1() {
        let input = read_input_lines_usize(&PathBuf::from(TEST_INPUT_FILE));
        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn test_part_2() {
        let input = read_input_lines_usize(&PathBuf::from(TEST_INPUT_FILE));
        assert_eq!(part2(&input), 5);
    }
}

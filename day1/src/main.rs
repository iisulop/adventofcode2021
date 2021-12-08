use std::path::PathBuf;

use common::{read_input_lines_usize, run};

fn part1(infile: &PathBuf) -> usize {
    let input = read_input_lines_usize(infile);
    input
        .iter()
        .fold((0, None), |(count, previous), current| match previous {
            Some(previous) if current > previous => (count + 1, Some(current)),
            Some(_) => (count, Some(current)),
            None => (count, Some(current)),
        })
        .0
}

fn part2(infile: &PathBuf) -> usize {
    let input = read_input_lines_usize(infile);
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
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::path::PathBuf;

    const TEST_INPUT_FILE: &str = "inputs/input_test";

    #[test]
    fn test_day_1_part_1() {
        let input = &PathBuf::from(TEST_INPUT_FILE);
        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn test_day_1_part_2() {
        let input = &PathBuf::from(TEST_INPUT_FILE);
        assert_eq!(part2(&input), 5);
    }
}

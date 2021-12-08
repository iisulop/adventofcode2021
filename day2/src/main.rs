use std::path::PathBuf;
use common::{init, read_input_lines_string};

struct Position {
    depth: usize,
    distance: usize,
    aim: usize,
}

impl Position {
    fn new() -> Self {
        Self{ depth: 0, distance: 0, aim: 0 }
    }
    fn result(&self) -> usize {
        self.depth * self.distance
    }
}

impl Position {
    fn forward(mut self, n: usize) -> Self {
        self.distance += n;
        self
    }
    fn down(mut self, n: usize) -> Self {
        self.depth += n;
        self
    }
    fn up(mut self, n: usize) -> Self {
        self.depth -= n;
        self
    }
}

pub fn part1(input: &PathBuf) -> usize {
    let input = read_input_lines_string(input);
    let end_pos = input.iter().fold(Position::new(), |pos, line| {
        match line.split(" ").collect::<Vec<&str>>().as_slice() {
            ["forward", num] => pos.forward(usize::from_str_radix(num, 10).unwrap()),
            ["down", num] => pos.down(usize::from_str_radix(num, 10).unwrap()),
            ["up", num] => pos.up(usize::from_str_radix(num, 10).unwrap()),
            e => panic!("Not supported: #{:?}", e)
        }
    });
    end_pos.result()
}

impl Position {
    fn move_forward(mut self, n: usize) -> Self {
        self.distance += n;
        self.depth += n*self.aim;
        self
    }
    fn increase_aim(mut self, n: usize) -> Self {
        self.aim += n;
        self
    }
    fn decrease_aim(mut self, n: usize) -> Self {
        self.aim -= n;
        self
    }
}

pub fn part2(input: &PathBuf) -> usize {
    let input = read_input_lines_string(input);
    let end_pos = input.iter().fold(Position::new(), |pos, line| {
        match line.split(" ").collect::<Vec<&str>>().as_slice() {
            ["forward", num] => pos.move_forward(usize::from_str_radix(num, 10).unwrap()),
            ["down", num] => pos.increase_aim(usize::from_str_radix(num, 10).unwrap()),
            ["up", num] => pos.decrease_aim(usize::from_str_radix(num, 10).unwrap()),
            e => panic!("Not supported: #{:?}", e)
        }
    });
    end_pos.result()
}

fn main() {
    let opt = init();
    if opt.part == 1 {
        let result_part1 = part1(&opt.infile);
        println!("Part 1: {}", result_part1);
    } else if opt.part == 2 {
        let result_part2 = part2(&opt.infile);
        println!("Part 2: {}", result_part2);
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    use std::path::PathBuf;

    const TEST_INPUT_FILE: &str = "inputs/input_test";

    #[test]
    fn test_day_2_part_1() {
        let input = PathBuf::from(TEST_INPUT_FILE);
        assert_eq!(part1(&input), 150);
    }

    #[test]
    fn test_day_2_part_2() {
        let input = PathBuf::from(TEST_INPUT_FILE);
        assert_eq!(part2(&input), 900);
    }
}

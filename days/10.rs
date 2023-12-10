use std::collections::VecDeque;

use itertools::Itertools;

struct Problem {
    problem: Vec<Vec<i32>>,
    max_height: usize,
    max_width: usize,
    start_value: (usize, usize),
}

impl Problem {
    pub fn get(&self, height: usize, width: usize) -> i32 {
        if height < self.max_height && width < self.max_width {
            return self.problem[height][width];
        }
        '.' as i32
    }

    pub fn new(input: Vec<Vec<i32>>) -> Self {
        let max_height = input.len();
        let max_width = input.first().unwrap().len();
        let mut start_value = (0, 0);
        for height in 0..max_height {
            for width in 0..max_width {
                if input[height][width] == 'S' as i32 {
                    start_value = (height, width);
                }
            }
        }

        Self {
            max_height,
            max_width,
            start_value,
            problem: input,
        }
    }

    pub fn get_firsts(&self) -> Vec<i32> {
        let result = Vec::<i32>::new();
        self.get(self.start_value.0 + 1, self.start_value.1 + 1);
        result
    }
}

fn solve_part1(input: &[String]) -> u64 {
    let input = input
        .iter()
        .map(|s| s.chars().map(|c| c as i32).collect_vec())
        .collect_vec();

    let problem = Problem::new(input);
    let mut queue = VecDeque::<(usize, usize)>::new();

    // queue.push_front(start);

    0
}

fn read_lines(input_file: &str) -> Vec<String> {
    std::fs::read_to_string(input_file)
        .unwrap()
        .trim_end()
        .replace("\r\n", "\n")
        .split('\n')
        .map(String::from)
        .collect()
}

fn main() {
    let input = read_lines("inputs/10.txt");
    let part1 = solve_part1(&input);
    println!("Part 1: {:?}", part1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let input: Vec<_> = ".....
.S-7.
.|.|.
.L-J.
....."
            .to_owned()
            .trim_end()
            .replace("\r\n", "\n")
            .split('\n')
            .map(String::from)
            .collect();

        let part1 = solve_part1(&input);
        assert_eq!(part1, 4);
    }

    #[test]
    fn test_more_complex() {
        let input: Vec<_> = "..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
            .to_owned()
            .trim_end()
            .replace("\r\n", "\n")
            .split('\n')
            .map(String::from)
            .collect();

        let part1 = solve_part1(&input);
        assert_eq!(part1, 8);
    }
}

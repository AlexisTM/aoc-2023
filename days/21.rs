use std::{collections::HashSet, mem::swap};

use pathfinding::matrix::{directions, Matrix};

type State = (usize, usize);

fn get_neighbours(state: &State, grid: &Matrix<char>) -> Vec<(State, u64)> {
    let mut result = Vec::<(State, u64)>::new();

    for dir in directions::DIRECTIONS_4.iter() {
        let next_position = grid.move_in_direction(*state, *dir);
        if let Some(next_position) = next_position {
            if let Some(c) = grid.get(next_position) {
                if *c == '#' {
                    continue;
                }
            }
            let mut next = state.clone();
            result.push((next_position, 1));
        }
    }

    result
}

fn find_all_destinations(
    start: (usize, usize),
    max_cost: u64,
    grid: &Matrix<char>,
) -> HashSet<(usize, usize)> {
    let mut current_set = HashSet::<(usize, usize)>::new();
    current_set.insert(start);
    let mut next_set = HashSet::<(usize, usize)>::new();

    for _ in 0..max_cost {
        for element in current_set.iter() {
            let neighbours = get_neighbours(element, grid);
            for neighbour in neighbours {
                next_set.insert(neighbour.0);
            }
        }
        swap(&mut current_set, &mut next_set);
        next_set = HashSet::<(usize, usize)>::new();
    }
    current_set
}

fn solve(input: &[String]) -> (u64, u64) {
    let grid = Matrix::from_rows(input.iter().map(|l| l.chars())).expect("Valid input");

    println!("problem {:?}", grid);

    let mut start: State = (0, 0);
    for (row_i, row) in grid.iter().enumerate() {
        for (col_i, c) in row.iter().enumerate() {
            if *c == 'S' {
                start = (row_i, col_i);
            }
        }
    }

    let results = find_all_destinations(start, 64, &grid);

    println!("results: {:?} \n\nnum: {}", results, results.len());

    let part1 = 0;

    // Divisors of 26501365: 1, 5, 11, 55, 481843, 2409215, 5300273, 26501365
    let part2 = 0;
    (part1, part2)
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
    let input = read_lines("inputs/21.txt");
    let (part1, part2) = solve(&input);
    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let input = read_lines("inputs/21_test.txt");

        let (part1, part2) = solve(&input);
        assert_eq!(part1, 0);
        assert_eq!(part2, 0);
    }
}

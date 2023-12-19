use pathfinding::matrix::{directions, Matrix};
use pathfinding::prelude::astar;

use std::hash::Hash;

use itertools::Itertools;

#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
struct State {
    position: (usize, usize),
    direction: (isize, isize),
    distance: u64,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Problem {
    maze: Vec<Vec<u8>>,
}

impl From<&[String]> for Problem {
    fn from(value: &[String]) -> Self {
        Problem {
            maze: value
                .iter()
                .map(|s| {
                    s.chars()
                        .map(|c| (c as i32 - '0' as i32) as u8)
                        .collect_vec()
                })
                .collect_vec(),
        }
    }
}

fn get_neighbours(state: &State, matrix: &Matrix<&u8>) -> Vec<(State, u64)> {
    let mut result = Vec::<(State, u64)>::new();

    for dir in directions::DIRECTIONS_4.iter() {
        let next_position = matrix.move_in_direction(state.position, *dir);
        if let Some(next_position) = next_position {
            let mut next = state.clone();

            if state.direction.0 == -dir.0 && state.direction.1 == -dir.1 {
                continue; // No backtrack
            }
            if state.direction == *dir {
                next.distance += 1;
                if state.distance == 3 {
                    continue;
                }
            } else {
                next.distance = 1;
                next.direction = *dir;
            }

            next.position = next_position;

            result.push((
                next,
                **matrix
                    .get(next_position)
                    .expect("The position has to exist") as u64,
            ));
        }
    }

    result
}

fn get_neighbours_p2(state: &State, matrix: &Matrix<&u8>) -> Vec<(State, u64)> {
    let mut result = Vec::<(State, u64)>::new();

    for dir in directions::DIRECTIONS_4.iter() {
        let next_position = matrix.move_in_direction(state.position, *dir);
        if let Some(next_position) = next_position {
            let mut next = state.clone();

            if state.direction.0 == -dir.0 && state.direction.1 == -dir.1 {
                continue; // No backtrack
            }
            if state.direction == *dir {
                next.distance += 1;
            } else {
                if next.distance < 4 {
                    continue;
                }
                next.distance = 1;
                next.direction = *dir;
            }

            next.position = next_position;
            if next.distance >= 10 {
                continue;
            }
            result.push((
                next,
                **matrix
                    .get(next_position)
                    .expect("The position has to exist") as u64,
            ));
        }
    }

    result
}

fn solve(input: &[String]) -> (u64, u64) {
    // I need the count of current movements (3 max)
    // Current direction
    // The position
    // The current count
    //
    // Keep the problem immutable?
    let problem = Problem::from(input);
    let matrix = Matrix::from_vec(
        problem.maze.len(),
        problem.maze[0].len(),
        problem.maze.iter().flatten().collect_vec(),
    )
    .expect("The matrix dimensions are invalid");

    println!("{:?}", matrix);

    let start = State {
        position: (0, 0),
        direction: directions::E,
        distance: 1,
    };

    let goal: (usize, usize) = (problem.maze.len() - 1, problem.maze[0].len() - 1);

    let result_p1 = astar(
        &start,
        |s| get_neighbours(s, &matrix), // successors
        |s| (goal.0.abs_diff(s.position.0) + goal.1.abs_diff(s.position.1)) as u64, // heuristic
        |s| s.position == goal,         // goal
    )
    .expect("Path to end exists");
    println!("result: {:?}", result_p1);
    let part1 = result_p1.1;

    let result_p2 = astar(
        &start,
        |s| get_neighbours_p2(s, &matrix), // successors
        |s| (goal.0.abs_diff(s.position.0) + goal.1.abs_diff(s.position.1)) as u64, // heuristic
        |s| s.position == goal && s.distance >= 4, // goal
    )
    .expect("Path to end exists");
    println!("result: {:?}", result_p2);
    // 1197

    let mut resulting_map = problem.maze.clone();

    for r in result_p2.0 {
        resulting_map[r.position.0][r.position.1] = 0;
        println!("{:?}", r.position);
    }

    for val in resulting_map.iter() {
        for c in val {
            print!("{c}");
        }
        println!();
    }
    let part2 = result_p2.1;
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
    let input = read_lines("inputs/17.txt");
    let (part1, part2) = solve(&input);
    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let input = read_lines("inputs/17_test.txt");

        let (part1, part2) = solve(&input);
        assert_eq!(part1, 102);
        assert_eq!(part2, 94);
    }

    #[test]
    fn test_more() {
        let input = "111111111111
999999999991
999999999991
999999999991
999999999991"
            .split('\n')
            .map(String::from)
            .collect_vec();

        let (_, part2) = solve(&input);
        assert_eq!(part2, 71);
    }
}

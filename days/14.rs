use itertools::Itertools;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

fn print_vec2(name: &str, vec: &[Vec<char>]) {
    println!("\n{name}");
    for s in vec.iter() {
        for c in s.iter() {
            print!("{c}");
        }
        println!()
    }
}

// Count in one pass;
fn count_north(input: &[Vec<char>]) -> usize {
    let mut max_weight = input.len();
    let mut cur_weights = vec![max_weight; input[0].len()];
    let mut result = 0;
    for (row_id, row) in input.iter().enumerate() {
        let row_strength = max_weight - row_id;
        for (col_id, val) in row.iter().enumerate() {
            match val {
                'O' => {
                    result += cur_weights[col_id];
                    cur_weights[col_id] -= 1;
                }
                '#' => {
                    cur_weights[col_id] = row_strength - 1;
                }
                _ => {}
            }
        }
    }
    result
}

// Similar as count_north, but modify the input
// Exactly what I didn't want to do for p1
fn tilt_north_inplace(input: &mut [Vec<char>]) {
    let mut cur_row_ids = vec![0; input[0].len()];

    let number_rows = input.len();
    let row_length = input[0].len();
    for row_id in 0..number_rows {
        for col_id in 0..row_length {
            match input[row_id][col_id] {
                'O' => {
                    if cur_row_ids[col_id] != row_id {
                        input[cur_row_ids[col_id]][col_id] = 'O';
                        input[row_id][col_id] = '.';
                    }
                    cur_row_ids[col_id] += 1;
                }
                '#' => {
                    cur_row_ids[col_id] = row_id + 1;
                }
                _ => {}
            }
        }
    }

    // result
}

fn rotate_board_90(board: &mut [Vec<char>]) {
    let mut tmp = vec![vec!['.'; board.len()]; board[0].len()];
    let n = board.len();

    for (i, row) in board.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            tmp[j][n - i - 1] = *c;
        }
    }
    for i in 0..board.len() {
        for j in 0..(board[0].len()) {
            board[i][j] = tmp[i][j];
        }
    }
}

fn hash(board: &[Vec<char>]) -> u64 {
    let mut hasher = DefaultHasher::new();

    board.hash(&mut hasher);
    hasher.finish()
}

fn compute_load(board: &[Vec<char>]) -> u64 {
    let max_weight = board.len();
    let mut result = 0;
    for (row_id, row) in board.iter().enumerate() {
        let row_strength = max_weight - row_id;
        for val in row.iter() {
            if *val == 'O' {
                result += row_strength;
            }
        }
    }
    result as u64
}

fn solve(input: &[String]) -> (u64, u64) {
    let mut input: Vec<Vec<char>> = input.iter().map(|s| s.chars().collect()).collect_vec();
    let part1 = count_north(input.as_slice());

    const MAX_STEPS: usize = 1_000_000_000;

    let mut step = 0;
    let mut seen = HashMap::<u64, usize>::new();
    while step < MAX_STEPS {
        for _ in 0..4 {
            tilt_north_inplace(input.as_mut_slice());
            rotate_board_90(input.as_mut_slice());
        }
        step += 1;
        let hash = hash(&input);
        let prev = *seen.entry(hash).or_insert(step);

        // Bypass all computation until right before the MAX_STEPS
        if prev < step {
            let cycle = step - prev;
            let full_cycles = (MAX_STEPS - step) / cycle;
            step += full_cycles * cycle;
        }
        println!("{step}");
    }

    (part1 as u64, compute_load(input.as_slice()))
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
    let input = read_lines("inputs/14.txt");
    let (part1, part2) = solve(&input);
    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let input = read_lines("inputs/14_test.txt");

        let (part1, part2) = solve(&input);
        assert_eq!(part1, 136);
        assert_eq!(part2, 64);
    }

    #[test]
    fn test_tilt_north() {
        let input = read_lines("inputs/14_test.txt");
        let mut input: Vec<Vec<char>> = input.iter().map(|s| s.chars().collect()).collect_vec();

        let expected_output = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."
            .trim_end()
            .replace("\r\n", "\n")
            .split('\n')
            .map(|s| s.chars().collect_vec())
            .collect_vec();

        let start = input.clone();
        tilt_north_inplace(input.as_mut_slice());
        let end = input.clone();
        assert_eq!(expected_output, end);
    }
    #[test]
    fn test_rotate_inplace() {
        let input = read_lines("inputs/14_test.txt");
        let mut input: Vec<Vec<char>> = input.iter().map(|s| s.chars().collect()).collect_vec();
        let start = input.clone();
        for _ in 0..3 {
            rotate_board_90(input.as_mut_slice());
            assert_ne!(start, input);
        }
        rotate_board_90(input.as_mut_slice());
        assert_eq!(start, input);
    }

    #[test]
    fn test_rotate_and_tilt() {
        let expected_1 = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."
            .trim_end()
            .replace("\r\n", "\n")
            .split('\n')
            .map(|s| s.chars().collect_vec())
            .collect_vec();
        let expected_2 = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"
            .trim_end()
            .replace("\r\n", "\n")
            .split('\n')
            .map(|s| s.chars().collect_vec())
            .collect_vec();
        let expected_3 = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"
            .trim_end()
            .replace("\r\n", "\n")
            .split('\n')
            .map(|s| s.chars().collect_vec())
            .collect_vec();

        let input = read_lines("inputs/14_test.txt");
        let mut input: Vec<Vec<char>> = input.iter().map(|s| s.chars().collect()).collect_vec();
        for _ in 0..4 {
            tilt_north_inplace(input.as_mut_slice());
            rotate_board_90(input.as_mut_slice());
        }
        assert_eq!(input, expected_1);
        for _ in 0..4 {
            tilt_north_inplace(input.as_mut_slice());
            rotate_board_90(input.as_mut_slice());
        }
        assert_eq!(input, expected_2);
        for _ in 0..4 {
            tilt_north_inplace(input.as_mut_slice());
            rotate_board_90(input.as_mut_slice());
        }
        assert_eq!(input, expected_3);
    }
}

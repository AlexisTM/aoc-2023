use std::cmp;

use itertools::Itertools;

fn find_galaxies(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut result = Vec::<(usize, usize)>::new();

    grid.iter().enumerate().for_each(|(row, s)| {
        s.iter().enumerate().for_each(|(col, c)| {
            if *c == '#' {
                result.push((row, col));
            }
        })
    });

    result
}

fn compute_expanded_distances(
    galaxies: &[(usize, usize)],
    row_expanded: &[usize],
    col_expanded: &[usize],
    expansion_ratio: usize,
) -> usize {
    return galaxies.iter().combinations(2).fold(0, |acc, a| {
        let g1 = a[0];
        let g2 = a[1];
        let min_row = cmp::min(g1.0, g2.0);
        let max_row = cmp::max(g1.0, g2.0);
        let min_col = cmp::min(g1.1, g2.1);
        let max_col = cmp::max(g1.1, g2.1);
        let row_count = row_expanded.iter().fold(0, |acc, row| {
            if *row < max_row && *row > min_row {
                acc + 1
            } else {
                acc
            }
        });
        let col_count = col_expanded.iter().fold(0, |acc, col| {
            if *col < max_col && *col > min_col {
                acc + 1
            } else {
                acc
            }
        });
        acc + g2.0.abs_diff(g1.0)
            + g2.1.abs_diff(g1.1)
            + row_count * expansion_ratio
            + col_count * expansion_ratio
    });
}

fn solve(input: &[String]) -> (u64, u64) {
    let grid: Vec<Vec<char>> = input.iter().map(|row| row.chars().collect()).collect();

    let mut row_expanded = Vec::<usize>::new();
    grid.iter().enumerate().for_each(|(index, s)| {
        if s.iter().all(|f| *f == '.') {
            row_expanded.push(index);
        }
    });

    let mut col_expanded = Vec::<usize>::new();
    let transposed_grid: Vec<Vec<char>> = (0..grid[0].len())
        .map(|col| grid.iter().map(|row| row[col]).collect())
        .collect();
    transposed_grid.iter().enumerate().for_each(|(index, s)| {
        if s.iter().all(|f| *f == '.') {
            col_expanded.push(index);
        }
    });

    let galaxies = find_galaxies(grid.as_slice());

    let part1 = compute_expanded_distances(&galaxies, &row_expanded, &col_expanded, 1);
    let part2 = compute_expanded_distances(&galaxies, &row_expanded, &col_expanded, 1000000 - 1);
    (part1 as u64, part2 as u64)
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
    let input = read_lines("inputs/11.txt");
    let (part1, part2) = solve(&input);
    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let input = read_lines("inputs/11_test.txt");

        let (part1, part2) = solve(&input);
        assert_eq!(part1, 374);
        assert_eq!(part2, 1030);
    }
}

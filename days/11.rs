use itertools::Itertools;

fn horizontal_expansion(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut horizontal_expansion = Vec::<Vec<char>>::new();
    input.iter().for_each(|s| {
        let empty_space = vec!['.'; s.len()]; //  str::repeat(".", s.len());
        horizontal_expansion.push(s.clone());
        if s.iter().all(|f| *f == '.') {
            horizontal_expansion.push(empty_space);
        }
    });

    horizontal_expansion
}

fn find_galaxies(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
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

fn solve(input: &[String]) -> (u64, u64) {
    let mut grid: Vec<Vec<char>> = input.iter().map(|row| row.chars().collect()).collect();
    let expanded = horizontal_expansion(&grid);
    let transposed_grid: Vec<Vec<char>> = (0..expanded[0].len())
        .map(|col| expanded.iter().map(|row| row[col]).collect())
        .collect();
    let expanded = horizontal_expansion(&transposed_grid);
    grid = (0..expanded[0].len())
        .map(|col| expanded.iter().map(|row| row[col]).collect())
        .collect();

    let galaxies = find_galaxies(&grid);
    println!("{:?}", galaxies);


    let part1 = galaxies.iter().combinations(2).fold(0, |acc, a| {
        let g1 = a[0];
        let g2 = a[1];
        acc + g2.0.abs_diff(g1.0)  + g2.1.abs_diff(g1.1) 
    });
    let part2 = 0;

    (part1 as u64, part2)
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
        assert_eq!(part2, 0);
    }
}

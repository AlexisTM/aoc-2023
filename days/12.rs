use itertools::Itertools;

/**
 * Checks if an input is valid.
 * ? are considered #
 */
fn check_valid(input: &str, points: &[u64]) -> bool {
    let val: Vec<_> = input
        .split('.')
        .filter(|f| !f.is_empty())
        .map(|f| f.len() as u64)
        .sorted_unstable()
        .collect_vec();

    println!("val {:?} vs points {:?}", val, points);
    if val.len() == points.len() {
        for i in 0..val.len() {
            if val[i] != points[i] {
                return false;
            }
        }
    }
    true
}

fn solve(input: &[String]) -> (u64, u64) {
    let count = 0;
    input.iter().for_each(|val| {
        let mut split = val.split(' ');
        let input = split.next().unwrap();
        let values: Vec<_> = split
            .last()
            .unwrap()
            .split(',')
            .map(|n| n.parse::<u64>().unwrap())
            .collect();
        let sum: u64 = values.iter().sum();
        let existing: u64 = input
            .chars()
            .fold(0, |acc, c| if c == '#' { acc + 1 } else { acc });
        let unknowns: u64 = input
            .chars()
            .fold(0, |acc, c| if c == '?' { acc + 1 } else { acc });

        println!(
            "{:?} {:?}, got {existing}, need {unknowns} = {sum}",
            input, values
        );
        check_valid(input, values.as_slice());
    });

    let part1 = 0;
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
    let input = read_lines("inputs/12.txt");
    let (part1, part2) = solve(&input);
    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let input = read_lines("inputs/12_test.txt");

        let (part1, part2) = solve(&input);
        assert_eq!(part1, 21);
        assert_eq!(part2, 0);
    }
}

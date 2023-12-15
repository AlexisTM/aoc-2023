fn solve(input: &[String]) -> (u64, u64) {
    let input = input.first().unwrap();
    let res = input.split(',').fold(0, |acc, s| {
        acc + s
            .chars()
            .fold(0, |sub_acc, c| ((sub_acc + (c as u64)) * 17) % 256)
    });
    let part1 = res;
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
    let input = read_lines("inputs/15.txt");
    let (part1, part2) = solve(&input);
    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let input = read_lines("inputs/15_test.txt");

        let (part1, part2) = solve(&input);
        assert_eq!(part1, 1320);
        assert_eq!(part2, 0);
    }
}

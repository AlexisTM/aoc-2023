fn solve(_input: &Vec<String>) -> (u64, u64) {
    let part1 = 0;
    let part2 = 0;
    (part1, part2)
}

fn main() {
    let input: Vec<String> = std::fs::read_to_string("inputs/00.txt")
        .unwrap()
        .trim_end()
        .split('\n')
        .map(String::from)
        .collect();

    let (part1, part2) = solve(&input);
    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let input: Vec<String> = std::fs::read_to_string("inputs/00_test.txt")
            .unwrap()
            .trim_end()
            .replace("\r\n", "\n") // Windows duh
            .split('\n')
            .map(String::from)
            .collect();

        let (part1, part2) = solve(&input);
        assert_eq!(part1, 0);
        assert_eq!(part2, 0);
    }
}

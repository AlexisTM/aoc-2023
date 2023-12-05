fn solve(input: &str) -> (i64, i64) {
    let p1 = input.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    });

    let mut p2_result = 0;
    let _p2 = input.chars().enumerate().fold(0, |acc, (index, c)| {
        let result = match c {
            '(' => acc + 1,
            ')' => acc - 1,
            _ => acc,
        };
        if result == -1 && p2_result == 0 {
            p2_result = index + 1;
        }
        result
    });

    let part1 = p1;
    let part2 = p2_result as i64;
    (part1, part2)
}

fn main() {
    let input: String = std::fs::read_to_string("inputs/01.txt").unwrap();

    let (part1, part2) = solve(&input);
    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}

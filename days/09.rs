use std::iter;

fn expander(line: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut problem = Vec::<Vec<i64>>::new();
    problem.push(line.clone());

    while !problem.last().unwrap().iter().all(|n| *n == 0) {
        let curr = problem.last().unwrap();
        let mut next = Vec::<i64>::new();

        for slice in curr.windows(2) {
            next.push(slice.last().unwrap() - slice.first().unwrap());
        }
        problem.push(next)
    }

    problem
}

fn extrapolation(problem: Vec<Vec<i64>>) -> i64 {
    let mut last = *problem.iter().rev().last().unwrap().last().unwrap();
    problem
        .iter()
        .rev()
        .fold(1, |acc, el| acc + *el.last().unwrap())
}

fn solve(input: &[String]) -> (i64, i64) {
    let input: Vec<Vec<i64>> = input
        .iter()
        .map(|s| {
            s.split(' ')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();

    let mut part1 = 0;
    for line in input.iter() {
        let res = expander(line);
        part1 += -1 + extrapolation(res);
    }

    let mut part2 = 0;
    for line in input.iter() {
        let mut p2_line = line.clone();
        p2_line.reverse();
        let res = expander(&p2_line);
        part2 += -1 + extrapolation(res);
    }

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
    let input = read_lines("inputs/09.txt");
    let (part1, part2) = solve(&input);
    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let input = read_lines("inputs/09_test.txt");

        let (part1, part2) = solve(&input);
        assert_eq!(part1, 114);
        assert_eq!(part2, 0);
    }
}

use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
struct Node<'a> {
    pub left: &'a str,
    pub right: &'a str,
}


fn part1(instructions: &str, source: &str, goal: &str,  map: &HashMap::<String, Node>) -> u64 {
    let mut curr: &str = source;
    let mut result = 0;
    'outer: loop {
        for instruction in instructions.chars() {
            let element = map.get(curr).unwrap();
            result += 1;
            match instruction {
                'L' => {
                    curr = element.left;
                }
                'R' => {
                    curr = element.right;
                }
                _ => {}
            }
            if curr == goal {
                break 'outer;
            }
        }
    }
    result
}

fn find_loop(instructions: &str, source: &str, map: &HashMap::<String, Node>) -> (u64, u64) {
    let mut curr: &str = source;
    let mut result = 0;
    let mut last_res = 0;
    let mut last_period = 0;
    loop {
        for instruction in instructions.chars() {
            let element = map.get(curr).unwrap();
            result += 1;
            match instruction {
                'L' => {
                    curr = element.left;
                }
                'R' => {
                    curr = element.right;
                }
                _ => {}
            }

            if curr.ends_with('Z') {
                let period = result - last_res;

                if period == last_period {
                    return (last_period, result);
                }
                last_res = result;
                last_period = period;
            }
        }
    }
}

// Greatest common divisor
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

// Least common multiple
pub fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn solve(input: &[String]) -> (u64, u64) {
    let instructions = input.first().unwrap();

    let mut map = HashMap::<String, Node>::new();
    input.iter().skip(2).for_each(|s| {
        let a: Vec<_> = s
            .split([';', '(', ' ', ')', '=', ','])
            .filter(|f| !f.is_empty())
            .collect();
        map.insert(
            a[0].to_owned(),
            Node {
                left: a[1],
                right: a[2],
            },
        );
    });

    // Naïve search
    let part1 = part1(instructions, "AAA", "ZZZ", &map);

    // Part2:
    // Find looping period
    // Compute the least common multiple
    let starting_nodes: Vec<_> = map
        .keys()
        .filter(|s| s.ends_with('A'))
        .map(String::to_owned)
        .collect();

    let periods = starting_nodes.iter().map(|node| {
        let (period, _) = find_loop(instructions, node, &map);
        period
    }).collect_vec();

    let part2 = lcm(periods.as_slice());

    (part1, part2)
}

fn main() {
    let input: Vec<String> = std::fs::read_to_string("inputs/08.txt")
        .unwrap()
        .trim_end()
        .replace("\r\n", "\n")
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
        let input: Vec<String> = std::fs::read_to_string("inputs/08_test.txt")
            .unwrap()
            .trim_end()
            .replace("\r\n", "\n") // Windows duh
            .split('\n')
            .map(String::from)
            .collect();

        let (part1, part2) = solve(&input);
        assert_eq!(part1, 2);
        assert_eq!(part2, 6);
    }
}

use std::collections::HashMap;

use itertools::Itertools;
use rayon::iter::Split;

#[derive(Debug)]
struct Condition {
    pub var: char,
    pub comp: char,
    pub value: u64,
    pub result: String, // A, R or stuff
}

impl From<&str> for Condition {
    fn from(value: &str) -> Self {
        let mut chars = value.chars();
        let mut val = value.split(['>', '<', ':']);

        Self {
            var: chars.next().unwrap(),
            comp: chars.next().unwrap(),
            value: val.nth(1).unwrap().parse().unwrap(),
            result: val.last().unwrap().to_owned(),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    pub identifier: String,
    pub conditions: Vec<Condition>,
    pub else_cond: String,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let mut split = value.split(['}', '{']);
        let identifier = split.next().expect("Expect a valid identifier");
        let instructs = split.next().expect("Expect instructions").split(',');
        let mut else_cond = String::new();
        let mut conditions = Vec::<Condition>::new();
        for instruct in instructs {
            if instruct.contains(':') {
                conditions.push(Condition::from(instruct));
            } else {
                else_cond = instruct.to_owned();
            }
        }

        Self {
            identifier: identifier.to_owned(),
            conditions,
            else_cond,
        }
    }
}

#[derive(Debug, Clone)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let mut val = value.split(['{', '}', ',', '=']);
        Self {
            // nth consumes all precedent values too
            x: val.nth(2).unwrap().parse().unwrap(),
            m: val.nth(1).unwrap().parse().unwrap(),
            a: val.nth(1).unwrap().parse().unwrap(),
            s: val.nth(1).unwrap().parse().unwrap(),
        }
    }
}

impl Part {
    pub fn work(&self, instruction: &Instruction) -> String {
        for cond in instruction.conditions.iter() {
            let var = match cond.var {
                'x' => self.x,
                'm' => self.m,
                'a' => self.a,
                's' => self.s,
                _ => {
                    println!("Shieze");
                    0
                }
            };
            let comp = match cond.comp {
                '>' => var > cond.value,
                '<' => var < cond.value,
                _ => false,
            };
            if comp {
                return cond.result.clone();
            }
        }
        instruction.else_cond.clone()
    }
}

fn solve(instructions_str: &[String], data_str: &[String]) -> (u64, u64) {
    let mut instructions = HashMap::<String, Instruction>::new();
    let mut parts = Vec::<Part>::new();

    let mut accepted = Vec::<Part>::new();
    for line in instructions_str.iter() {
        let instruction = Instruction::from(line.as_str());
        instructions.insert(instruction.identifier.to_owned(), instruction);
    }
    for line in data_str.iter() {
        parts.push(Part::from(line.as_str()));
    }

    for part in parts.iter() {
        println!("#part: {:?}", part);
        let mut current_instr = "in".to_string();
        loop {
            println!("Instruction: {}", current_instr);
            match current_instr.as_str() {
                "A" => {
                    accepted.push(part.clone());
                    break;
                }
                "R" => {
                    break;
                }
                _ => {}
            }
            let instruction = instructions.get(&current_instr).expect("instruction");
            current_instr = part.work(instruction);
        }
    }

    let part1 = accepted.iter().map(|p| p.x + p.m + p.a + p.s).sum();
    println!("{:?}", accepted);
    println!("{:?}", parts);
    let part2 = 0;
    (part1, part2)
}

fn read_lines(input_file: &str) -> Vec<Vec<String>> {
    std::fs::read_to_string(input_file)
        .unwrap()
        .trim_end()
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|part| part.split('\n').map(String::from).collect_vec())
        .collect_vec()
}

fn main() {
    let input = read_lines("inputs/19.txt");
    let (part1, part2) = solve(input.first().unwrap(), input.last().unwrap());
    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let input = read_lines("inputs/19_test.txt");

        let (part1, part2) = solve(input.first().unwrap(), input.last().unwrap());
        assert_eq!(part1, 19114);
        assert_eq!(part2, 0);
    }
}

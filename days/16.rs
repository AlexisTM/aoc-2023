use std::fmt::{Display, Write};

use itertools::Itertools;

#[derive(Debug, Clone)]
enum Direction {
    Top,
    Left,
    Bottom,
    Right,
}

#[derive(Debug, Clone)]
struct Node {
    pub val: char,
    pub seen_t: bool,
    pub seen_b: bool,
    pub seen_l: bool,
    pub seen_r: bool,
}

#[derive(Debug, Clone)]
struct Problem {
    pub maze: Vec<Vec<Node>>,
}

impl Problem {
    pub fn get(&mut self, x: i64, y: i64) -> Option<&mut Node> {
        if (x as usize) < self.maze.len() && (y as usize) < self.maze[0].len() {
            Some(&mut self.maze[x as usize][y as usize])
        } else {
            None
        }
    }
}

impl From<&[String]> for Problem {
    fn from(value: &[String]) -> Self {
        Problem {
            maze: value
                .iter()
                .map(|s| {
                    s.chars()
                        .map(|c| Node {
                            val: c,
                            seen_t: false,
                            seen_b: false,
                            seen_l: false,
                            seen_r: false,
                        })
                        .collect_vec()
                })
                .collect_vec(),
        }
    }
}

impl Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.maze.iter() {
            for node in row.iter() {
                if node.seen_r || node.seen_l || node.seen_t || node.seen_b {
                    f.write_char('#')?;
                } else {
                    f.write_char(node.val)?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn energize(problem: &mut Problem, initial_pose: (i64, i64, Direction)) -> u64 {
    let mut deque = vec![initial_pose];
    while let Some((x, y, to)) = deque.pop() {
        // println!("Visiting {x} {y} {:?}", to);
        if let Some(node) = &mut problem.get(x, y) {
            match to {
                Direction::Bottom => {
                    if node.seen_b {
                        continue;
                    }
                    node.seen_b = true;
                    match node.val {
                        '/' => deque.push((x, y - 1, Direction::Left)),
                        '\\' => deque.push((x, y + 1, Direction::Right)),
                        '-' => {
                            deque.push((x, y + 1, Direction::Right));
                            deque.push((x, y - 1, Direction::Left));
                        }
                        _ => deque.push((x + 1, y, to)),
                    };
                }
                Direction::Top => {
                    if node.seen_t {
                        continue;
                    }
                    node.seen_t = true;
                    match node.val {
                        '/' => deque.push((x, y + 1, Direction::Right)),
                        '\\' => deque.push((x, y - 1, Direction::Left)),
                        '-' => {
                            deque.push((x, y + 1, Direction::Right));
                            deque.push((x, y - 1, Direction::Left));
                        }
                        _ => deque.push((x - 1, y, to)),
                    };
                }
                Direction::Left => {
                    if node.seen_l {
                        continue;
                    }
                    node.seen_l = true;
                    match node.val {
                        '/' => deque.push((x + 1, y, Direction::Bottom)),
                        '\\' => deque.push((x - 1, y, Direction::Top)),
                        '|' => {
                            deque.push((x + 1, y, Direction::Bottom));
                            deque.push((x - 1, y, Direction::Top));
                        }
                        _ => deque.push((x, y - 1, to)),
                    };
                }
                Direction::Right => {
                    if node.seen_r {
                        continue;
                    }
                    node.seen_r = true;
                    match node.val {
                        '/' => deque.push((x - 1, y, Direction::Top)),
                        '\\' => deque.push((x + 1, y, Direction::Bottom)),
                        '|' => {
                            deque.push((x + 1, y, Direction::Bottom));
                            deque.push((x - 1, y, Direction::Top));
                        }
                        _ => deque.push((x, y + 1, to)),
                    };
                }
            };
        }
    }

    return problem.maze.iter().fold(0, |acc, m| {
        acc + m
            .iter()
            .filter(|n| n.seen_r || n.seen_l || n.seen_t || n.seen_b)
            .count()
    }) as u64;
}

fn solve(input: &[String]) -> (u64, u64) {
    let mut problem: Problem = input.into();
    let part1 = energize(&mut problem, (0i64, 0i64, Direction::Right));

    // Just test all possibilities
    let problem: Problem = input.into();
    let mut results = Vec::<_>::new();
    for col in 0..problem.maze[0].len() {
        results.push(energize(
            &mut problem.clone(),
            (0, col as i64, Direction::Bottom),
        ));
        results.push(energize(
            &mut problem.clone(),
            (problem.maze.len() as i64, col as i64, Direction::Top),
        ));
    }
    for row in 0..problem.maze.len() {
        results.push(energize(
            &mut problem.clone(),
            (row as i64, 0, Direction::Right),
        ));
        results.push(energize(
            &mut problem.clone(),
            (row as i64, problem.maze[0].len() as i64, Direction::Left),
        ));
    }

    let part2 = *results.iter().max().unwrap();
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
    let input = read_lines("inputs/16.txt");
    let (part1, part2) = solve(&input);
    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let input = read_lines("inputs/16_test.txt");

        let (part1, part2) = solve(&input);
        assert_eq!(part1, 46);
        assert_eq!(part2, 51);
    }
}

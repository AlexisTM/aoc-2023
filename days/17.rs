use std::collections::VecDeque;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Top,
    Left,
    Bottom,
    Right,
}

#[derive(Debug, Clone)]
struct Search<'a> {
    x: i64,
    y: i64,
    curr: u64,
    to: Direction,
    to_count: u8,
    path: Vec<(i64, i64)>,
    problem: &'a Problem,
}

impl<'a> Search<'a> {
    pub fn create(&self, to: Direction) -> Search<'a> {
        let mut other = self.clone();
        other.to = to.clone();

        match &to {
            Direction::Left => other.y -= 1,
            Direction::Right => other.y += 1,
            Direction::Top => other.x -= 1,
            Direction::Bottom => other.x += 1,
        }
        other.path.push((self.x, self.y));
        if let Some(val) = self.problem.get(other.x, other.y) {
            other.curr += val as u64;
        }

        if other.to == self.to {
            other.to_count += 1;
        } else {
            other.to_count = 1;
        }
        other
    }

    pub fn valid(&self) -> bool {
        self.x >= 0
            && self.y >= 0
            && self.x < self.problem.maze.len() as i64
            && self.y < self.problem.maze.len() as i64
            && self.to_count <= 3
            && !self.path.contains(&(self.x, self.y))
    }

    pub fn next(&self, vec: &mut VecDeque<Search<'a>>) {
        let nexts = vec![
            self.create(Direction::Left),
            self.create(Direction::Right),
            self.create(Direction::Bottom),
            self.create(Direction::Top),
        ];

        for next in nexts {
            if next.valid() {
                vec.push_back(next);
            }
        }
    }
}

#[derive(Debug)]
struct Problem {
    maze: Vec<Vec<u8>>,
}

impl Problem {
    pub fn get(&self, x: i64, y: i64) -> Option<u8> {
        if (x as usize) < self.maze.len() && (y as usize) < self.maze[0].len() {
            Some(self.maze[x as usize][y as usize])
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
                        .map(|c| (c as i32 - '0' as i32) as u8)
                        .collect_vec()
                })
                .collect_vec(),
        }
    }
}

fn solve(input: &[String]) -> (u64, u64) {
    // I need the count of current movements (3 max)
    // Current direction
    // The position
    // The current count
    // Handle multiple solutions at the same time?
    //
    // Cascading solution? Solving each 3x3 or 10x10 as an anyedge to any edge providing a new smaller problem?
    // This causes the 'next problem' to have initial conditions such as direction & number of
    // steps
    //
    // Keep the problem immutable?
    let problem = Problem::from(input);

    let search = Search {
        x: 0,
        y: 0,
        curr: problem.get(0, 0).unwrap() as u64,
        to: Direction::Right,
        to_count: 1,
        path: Vec::new(),
        problem: &problem,
    };

    let mut vec = VecDeque::<Search>::new();
    vec.push_back(search);

    let mut best_result: Option<Search> = None;
    while let Some(search) = vec.pop_back() {
        if let Some(result) = &best_result {
            // prune bad routes
            if search.curr >= result.curr {
                continue;
            }
        } else {
            //println!("Searches: {}", vec.len());
        }

        if search.x == problem.maze.len() as i64 - 1 && search.y == problem.maze.len() as i64 - 1 {
            if let Some(result) = &best_result {
                if search.curr < result.curr {
                    println!("Searches: {}    curr {} ", vec.len(), result.curr);
                    best_result = Some(search);
                }
            } else {
                best_result = Some(search);
            }
            continue;
        } else {
            search.next(&mut vec);
        }
    }

    println!("{:?}", best_result);
    let mut part1 = best_result.unwrap().curr;
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
    let input = read_lines("inputs/17.txt");
    let (part1, part2) = solve(&input);
    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let input = read_lines("inputs/17_test.txt");

        let (part1, part2) = solve(&input);
        assert_eq!(part1, 102);
        assert_eq!(part2, 0);
    }
}

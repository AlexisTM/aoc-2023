use itertools::Itertools;

#[derive(Debug)]
enum Direction {
    Top,
    Bot,
    Left,
    Right,
    None,
}

#[derive(Debug)]
struct Problem {
    problem: Vec<Vec<char>>,
    max_height: i64,
    max_width: i64,
    start_value: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    pub h: i64,
    pub w: i64,
    pub val: char,
}

#[derive(Debug, Clone)]
struct Search {
    pub path: Vec<Position>,
    pub pos: Position,
    pub steps: i32,
    pub id: char,
}

fn print_vec2(name: &str, vec: &[Vec<char>]) {
    println!("\n{name}");
    for s in vec.iter() {
        for c in s.iter() {
            print!("{c}");
        }
        println!()
    }
}

impl Problem {
    pub fn get(&self, height: i64, width: i64) -> Position {
        if height >= 0 && height < self.max_height && width >= 0 && width < self.max_width {
            return Position {
                h: height,
                w: width,
                val: self.problem[height as usize][width as usize],
            };
        }
        Position {
            h: height,
            w: width,
            val: 'Z',
        }
    }

    pub fn new(input: Vec<Vec<char>>) -> Self {
        let max_height = input.len() as i64;
        let max_width = input.first().unwrap().len() as i64;
        let mut start_value = Position {
            h: 0,
            w: 0,
            val: '.',
        };
        for height in 0..max_height {
            for width in 0..max_width {
                if input[height as usize][width as usize] == 'S' {
                    start_value = Position {
                        h: height,
                        w: width,
                        val: 'S',
                    };
                }
            }
        }

        Self {
            max_height,
            max_width,
            start_value,
            problem: input,
        }
    }

    pub fn navigate(&mut self, search: &mut Search, to: Direction) -> Direction {
        //println!("Handling {}, got {}", search.id, search.pos.val);
        search.path.push(search.pos);
        match to {
            Direction::Top => match search.pos.val {
                '7' => Direction::Left,
                'F' => Direction::Right,
                '|' => Direction::Top,
                _ => Direction::None,
            },
            Direction::Bot => match search.pos.val {
                'J' => Direction::Left,
                'L' => Direction::Right,
                '|' => Direction::Bot,
                _ => Direction::None,
            },
            Direction::Left => match search.pos.val {
                'L' => Direction::Top,
                '-' => Direction::Left,
                'F' => Direction::Bot,
                _ => Direction::None,
            },
            Direction::Right => match search.pos.val {
                '-' => Direction::Right,
                'J' => Direction::Top,
                '7' => Direction::Bot,
                _ => Direction::None,
            },
            Direction::None => Direction::None,
        }
    }

    pub fn go_through(&mut self, search: &mut Search, to: Direction) -> Search {
        let next_pose = match to {
            Direction::Top => (search.pos.h - 1, search.pos.w),
            Direction::Bot => (search.pos.h + 1, search.pos.w),
            Direction::Left => (search.pos.h, search.pos.w - 1),
            Direction::Right => (search.pos.h, search.pos.w + 1),
            Direction::None => return search.clone(),
        };
        // println!("Navigating from {:?} towards {:?}", search, to);
        search.steps += 1;
        search.pos = self.get(next_pose.0, next_pose.1);

        let mut to = self.navigate(search, to);

        while search.pos.val != 'S' && search.pos.val != '.' {
            let next_pose = match to {
                Direction::Top => (search.pos.h - 1, search.pos.w),
                Direction::Bot => (search.pos.h + 1, search.pos.w),
                Direction::Left => (search.pos.h, search.pos.w - 1),
                Direction::Right => (search.pos.h, search.pos.w + 1),
                Direction::None => return search.clone(),
            };
            // println!("Navigating from {:?} towards {:?}", search, to);
            search.steps += 1;
            search.pos = self.get(next_pose.0, next_pose.1);

            to = self.navigate(search, to);
        }
        search.clone()
    }

    pub fn navigate_all(&mut self) -> Vec<Search> {
        let result = vec![
            self.go_through(
                &mut Search {
                    pos: self.start_value,
                    steps: 0,
                    id: '1',
                    path: vec![],
                },
                Direction::Top,
            ),
            self.go_through(
                &mut Search {
                    pos: self.start_value,
                    steps: 0,
                    id: '2',
                    path: vec![],
                },
                Direction::Bot,
            ),
            self.go_through(
                &mut Search {
                    pos: self.start_value,
                    steps: 0,
                    id: '3',
                    path: vec![],
                },
                Direction::Left,
            ),
            self.go_through(
                &mut Search {
                    pos: self.start_value,
                    steps: 0,
                    id: '4',
                    path: vec![],
                },
                Direction::Right,
            ),
        ];
        result
    }
}

fn solve_part1(input: &[String]) -> u64 {
    let input = input.iter().map(|s| s.chars().collect_vec()).collect_vec();

    let mut problem = Problem::new(input);

    let results = problem.navigate_all();
    for s in results.iter() {
        println!("Results: {:?}", s);
    }
    println!();
    let result = results
        .iter()
        .filter(|s| s.pos.val == 'S')
        .max_by(|s1, s2| s1.steps.cmp(&s2.steps))
        .unwrap();
    println!("Results: {:?}", result);

    let mut part2 = 0;
    // part2
    problem.problem.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, col)| {
            if *col == '|' {
                return;
            }
            for res in results.iter() {
                for r in res.path.iter() {
                    if r.h == i as i64 && r.w == j as i64 {
                        return;
                    }
                }
            }

            // Ray trace to 0, if odd interesction: good
            let mut crosses = 0;
            for ray in 0..i {
                for r in result.path.iter() {
                    if r.h == ray as i64 && r.w == j as i64 {
                        crosses += 1;
                    }
                }
            }
            if crosses & 1 == 1 {
                part2 += 1;
                println!("ray [{i}][{j}]");
                println!("CROSSES: {crosses}");
            }
        })
    });

    result.steps as u64 / 2u64 + (result.steps as u64 & 1u64)
}

fn solve_part2(input: &[String]) -> u64 {
    let input = input.iter().map(|s| s.chars().collect_vec()).collect_vec();

    let mut problem = Problem::new(input);

    let results = problem.navigate_all();
    for s in results.iter() {
        println!("Results: {:?}", s);
    }
    println!();
    let result = results
        .iter()
        .filter(|s| s.pos.val == 'S')
        .max_by(|s1, s2| s1.steps.cmp(&s2.steps))
        .unwrap();
    println!("Results: {:?}", results);

    print_vec2("RESULT", &problem.problem);
    let mut part2 = 0;
    // part2
    problem.problem.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, _col)| {
            for res in results.iter() {
                for r in res.path.iter() {
                    if r.h == i as i64 && r.w == j as i64 {
                        return;
                    }
                }
            }

            // Ray trace to 0, if odd interesction: good
            let mut crosses = 0;
            for ray in 0..i {
                for r in result.path.iter() {
                    if r.h == ray as i64 && r.w == j as i64 && problem.problem[ray][j] != '|' {
                        crosses += 1;
                        if i == 8 && j == 1 {
                            println!("ray [{ray}][{j}]");
                        }
                    }
                }
            }
            if crosses & 1 == 1 {
                part2 += 1;
                println!("ray [{i}][{j}]");
                println!("CROSSES: {crosses}");
            }
        })
    });

    println!("RESULT ==== {part2}");
    part2
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
    let input = read_lines("inputs/10.txt");
    let part1 = solve_part1(&input);
    // solve_part2(&input);
    println!("Part 1: {:?}", part1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let input: Vec<_> = ".....
.S-7.
.|.|.
.L-J.
....."
            .to_owned()
            .trim_end()
            .replace("\r\n", "\n")
            .split('\n')
            .map(String::from)
            .collect();

        let part1 = solve_part1(&input);
        assert_eq!(part1, 4);
    }

    #[test]
    fn test_more_complex() {
        let input: Vec<_> = "..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
            .to_owned()
            .trim_end()
            .replace("\r\n", "\n")
            .split('\n')
            .map(String::from)
            .collect();

        let part1 = solve_part1(&input);
        assert_eq!(part1, 8);
    }

    #[test]
    fn test_part2_simple() {
        let input: Vec<_> = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
            .to_owned()
            .trim_end()
            .replace("\r\n", "\n")
            .split('\n')
            .map(String::from)
            .collect();

        let part2 = solve_part2(&input);
        assert_eq!(part2, 4);
    }

    #[test]
    fn test_part2_complex() {
        let input: Vec<_> = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
            .to_owned()
            .trim_end()
            .replace("\r\n", "\n")
            .split('\n')
            .map(String::from)
            .collect();

        let part2 = solve_part2(&input);
        assert_eq!(part2, 8);
    }
}

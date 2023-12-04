struct Problem {
    height: i16,
    width: i16,
    problem_data: Vec<char>,
}

impl Problem {
    pub fn new(input: &Vec<String>) -> Self {
        let height = input.len();
        let width = input.first().unwrap().len();
        let mut problem_data = Vec::<char>::default();
        problem_data.resize(width * height, '.');
        for (h, data) in input.iter().enumerate() {
            for (w, c) in data.chars().enumerate() {
                problem_data[h * width + w] = c;
            }
        }

        Problem {
            height: height as i16,
            width: width as i16,
            problem_data,
        }
    }

    pub fn get(&self, height: i16, width: i16) -> char {
        if height < 0 || width < 0 || height >= self.height || width >= self.width {
            return '.';
        }
        self.problem_data[(height * self.width + width) as usize]
    }

    pub fn has_special_chars(&self, h_pos: i16, w_pos: i16) -> bool {
        for h in (h_pos - 1)..(h_pos + 2) {
            for w in (w_pos - 1)..(w_pos + 2) {
                let c = self.get(h, w);
                if c != '.' && !c.is_numeric() {
                    return true;
                }
            }
        }
        false
    }

    pub fn get_sum_parts(&self) -> u64 {
        let mut result = 0;
        for h in 0..(self.height) {
            let mut number_str = String::new();
            let mut validated = false;
            for w in 0..self.width + 1 {
                // Will trigger to find a '.' and trigger the number validation
                let c = self.get(h, w);
                if c.is_numeric() {
                    number_str.push(c);
                    if !validated {
                        validated = self.has_special_chars(h, w);
                    }
                } else {
                    if validated {
                        let val = number_str.parse::<u64>().unwrap();
                        result += val;
                    }
                    number_str = String::new();
                    validated = false;
                    // Make number, check if number validated, reset number
                    continue;
                }
            }
        }
        result
    }

    // Returns the (location and parsed number)
    pub fn get_number_at_pos(&self, h_pos: i16, w_pos: i16) -> (i16, u64) {
        let mut now_w_pos = w_pos;
        while self.get(h_pos, now_w_pos).is_numeric() {
            now_w_pos -= 1;
        }
        now_w_pos += 1;
        let start_w_pos = now_w_pos;

        let mut number = String::new();
        let mut c = self.get(h_pos, now_w_pos);
        while c.is_numeric() {
            number.push(c);
            now_w_pos += 1;
            c = self.get(h_pos, now_w_pos);
        }
        (
            self.width * h_pos + start_w_pos,
            number.parse::<u64>().unwrap(),
        )
    }

    pub fn find_digits_around_gear(&self, h_pos: i16, w_pos: i16) -> Option<(u64, u64)> {
        let mut all_numbers = Vec::<(i16, u64)>::new();
        for h in (h_pos - 1)..(h_pos + 2) {
            for w in (w_pos - 1)..(w_pos + 2) {
                let c = self.get(h, w);
                if c.is_numeric() {
                    let pair = self.get_number_at_pos(h, w);
                    if all_numbers.iter().all(|(index, _)| *index != pair.0) {
                        all_numbers.push(pair);
                    }
                }
            }
        }
        if all_numbers.len() == 2 {
            return Some((all_numbers[0].1, all_numbers[1].1));
        }
        None
    }

    pub fn get_sum_of_gears(&self) -> u64 {
        // We will look for gears, add the multiplication of both and remove both form the part1.
        let mut result = 0;
        for h in 0..(self.height) {
            let _number_str = String::new();
            let _validated = false;
            for w in 0..self.width + 1 {
                let c = self.get(h, w);
                if c == '*' {
                    // We got a gear
                    let pair = self.find_digits_around_gear(h, w);
                    if let Some(pair) = pair {
                        result += pair.0 * pair.1;
                    }
                }
            }
        }
        result
    }
}

fn solve(input: &Vec<String>) -> (u64, u64) {
    let _part2 = 0;

    let problem = Problem::new(input);
    let part1 = problem.get_sum_parts();
    let gears_value = problem.get_sum_of_gears();
    let part2 = gears_value;
    (part1, part2)
}

fn main() {
    let input: Vec<String> = std::fs::read_to_string("inputs/03.txt")
        .unwrap()
        .trim_end()
        .replace("\r\n", "\n") // Windows duh
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
        let input: Vec<String> = std::fs::read_to_string("inputs/03_test.txt")
            .unwrap()
            .trim_end()
            .replace("\r\n", "\n") // Windows duh
            .split('\n')
            .map(String::from)
            .collect();

        let (part1, part2) = solve(&input);
        assert_eq!(part1, 4361);
        assert_eq!(part2, 467835);
    }
}

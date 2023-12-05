#[derive(Debug)]
struct Map {
    destination: i64,
    source: i64,
    length: i64,
}

// Location mapping from one area to another
#[derive(Debug)]
struct LocMapping {
    pub maps: Vec<Map>,
}

impl Map {
    pub fn new(input: &str) -> Self {
        let input: Vec<&str> = input.split(' ').collect();
        if input.len() != 3 {
            println!("Incorrect data... {:?}", input);
        }
        Self {
            destination: input[0].parse().unwrap(),
            source: input[1].parse().unwrap(),
            length: input[2].parse().unwrap(),
        }
    }
}

impl LocMapping {
    pub fn new() -> Self {
        Self {
            maps: Vec::<Map>::new(),
        }
    }

    pub fn convert(&self, from: i64) -> i64 {
        for map in self.maps.iter() {
            if from >= map.source && from < map.source + map.length {
                return map.destination + from - map.source;
            }
        }
        from
    }
}

fn solve(input: &[String]) -> (i64, i64) {
    let input_seeds: Vec<i64> = input
        .first()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| {
            println!("{:?}", s);
            s.parse::<i64>().unwrap()
        })
        .collect();
    println!("{:?}", input_seeds);

    let mut maps = Vec::<LocMapping>::new();
    input.iter().skip(2).for_each(|s| {
        println!("Handling s: {:?}", s);
        if s.is_empty() {
        } else if s.chars().next().unwrap().is_alphabetic() {
            maps.push(LocMapping::new());
        } else {
            maps.last_mut().unwrap().maps.push(Map::new(s));
        }
    });
    // If not mapped => Same number
    println!("{:?}", maps);

    let output_seeds: Vec<i64> = input_seeds
        .iter()
        .map(|seed| {
            let mut result = *seed;
            for locmap in maps.iter() {
                println!("curr_seed: {result} for locmap: {:?}", locmap);
                result = locmap.convert(result);
                println!("gives: {result}");
            }
            result
        })
        .collect();

    println!("{:?}", output_seeds);

    let part1 = *output_seeds.iter().min().unwrap();
    let part2 = 0;
    (part1, part2)
}

fn main() {
    let input: Vec<String> = std::fs::read_to_string("inputs/05.txt")
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
        let input: Vec<String> = std::fs::read_to_string("inputs/05_test.txt")
            .unwrap()
            .trim_end()
            .replace("\r\n", "\n") // Windows duh
            .split('\n')
            .map(String::from)
            .collect();

        let (part1, part2) = solve(&input);
        assert_eq!(part1, 35);
        assert_eq!(part2, 0);
    }
}

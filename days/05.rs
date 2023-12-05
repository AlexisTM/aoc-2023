#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Map {
    source: i64,
    destination: i64,
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
            source: input[1].parse().unwrap(),
            destination: input[0].parse().unwrap(),
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

    pub fn sort(&mut self) {
        self.maps.sort()
    }

    pub fn convert_range(&self, from: i64, length: i64) -> Vec<i64> {
        let to = from + length; // To NOT inclusive
        let mut curr_from = from;
        let mut result = Vec::<i64>::new();
        for map in self.maps.iter() {
            // while long time:
            if curr_from == to {
                println!("input finished curr == to");
                return result;
            }
            if curr_from < map.source {
                result.push(curr_from);
                if to < map.source {
                    result.push(to - curr_from);
                    println!(
                        "curr < source and to < source => {curr_from} {}",
                        result.last().unwrap()
                    );
                    return result;
                } else {
                    result.push(map.source - curr_from);
                    println!(
                        "curr < source BUT to > source => limited {curr_from} {}",
                        result.last().unwrap()
                    );
                    curr_from = map.source;
                }
            } else {
                // curr >= map.source
                if curr_from > map.source + map.length {
                    println!("Our source is after this mapping, see next mapping");
                    continue; // Check next map
                }
                // Curr is within the mapping
                result.push(self.convert(curr_from));
                // Up to where?
                if to < map.source + map.length {
                    result.push(to - curr_from);
                    println!(
                        "curr >= source and to < end => full inside {curr_from} {}",
                        result.last().unwrap()
                    );
                    curr_from = to;
                } else {
                    println!(
                        "for below: ({} + {length}) - {curr_from} {to} = ...",
                        map.source
                    );
                    result.push((map.source + map.length) - curr_from);
                    println!(
                        "curr >= source but to > end; => finish this mapping fully {curr_from} {}",
                        result.last().unwrap()
                    );
                    curr_from = map.source + map.length;
                }
            }
        }

        if curr_from != to {
            result.push(curr_from);
            result.push(to - curr_from);
            println!("Finishing with {curr_from} and {}", result.last().unwrap());
        }
        result
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

    let mut maps = Vec::<LocMapping>::new();
    input.iter().skip(2).for_each(|s| {
        if s.is_empty() {
        } else if s.chars().next().unwrap().is_alphabetic() {
            maps.push(LocMapping::new());
        } else {
            maps.last_mut().unwrap().maps.push(Map::new(s));
        }
    });

    // If not mapped => Same number
    let output_seeds: Vec<i64> = input_seeds
        .iter()
        .map(|seed| {
            let mut result = *seed;
            for locmap in maps.iter() {
                result = locmap.convert(result);
            }
            result
        })
        .collect();

    let part1 = *output_seeds.iter().min().unwrap();

    // part 2;
    // Sort all LocMappings to make it more trivial to process
    maps.iter_mut().for_each(LocMapping::sort);
    let mut curr_seeds = input_seeds.clone();
    let mut next_seeds = Vec::<i64>::new();

    // For each locmap, run the
    maps.iter().for_each(|locmap| {
        println!("LOCMAP: {:?}", locmap);
        for seed in curr_seeds.chunks(2) {
            let new_seeds = locmap.convert_range(*seed.first().unwrap(), *seed.last().unwrap());
            next_seeds.extend(new_seeds);
        }
        println!("NEW_SEED_PAIRS: {:?}", next_seeds);

        curr_seeds = next_seeds.clone();
        next_seeds = Vec::<i64>::new();
    });
    println!("LAST_SEED_PAIRS: {:?}", curr_seeds);

    let mut part2 = i64::MAX;
    curr_seeds
        .chunks(2)
        .map(|pair| {
            let val = *pair.first().unwrap();
            if val < part2 {
                part2 = val;
            }
        })
        .count();

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
        assert_eq!(part2, 46);
    }

    #[test]
    fn test_locmapping_range() {
        let mut locmaps = LocMapping::new();
        locmaps.maps.push(Map {
            source: 50,
            destination: 52,
            length: 48,
        });
        locmaps.maps.push(Map {
            source: 98,
            destination: 50,
            length: 2,
        });
        let new_seeds = locmaps.convert_range(79, 14);
        assert_eq!(new_seeds.len(), 2);
        assert_eq!(new_seeds[0], 81);
        assert_eq!(new_seeds[1], 14);
        let new_seeds = locmaps.convert_range(55, 13);
        assert_eq!(new_seeds.len(), 2);
        assert_eq!(new_seeds[0], 57);
        assert_eq!(new_seeds[1], 13);
    }
}

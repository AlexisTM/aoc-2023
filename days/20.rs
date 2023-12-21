use itertools::Itertools;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::ops::Mul;
use std::{collections::HashMap, rc::Rc};

#[derive(Debug)]
enum ModuleVersion {
    Broadcaster, // Forwards LOW to all
    Conjonction, // & If all inputs are HIGH => LOW else HIGH
    FlipFlop,    // % If LOW: sends HIGH
    Unknown,
}

// state false == low pulse
// Pulse order is order sent;

#[derive(Debug)]
struct Module {
    version: ModuleVersion,
    inputs: Vec<String>,
    outputs: Vec<String>,
    last_state: bool,
}

impl Module {
    pub fn update(
        &mut self,
        high: bool,
        config: &HashMap<&str, Rc<RefCell<Module>>>,
    ) -> Option<bool> {
        match self.version {
            ModuleVersion::Broadcaster => return Some(false),
            ModuleVersion::Conjonction => {
                if self.inputs.iter().all(|element| {
                    config
                        .get(element.as_str())
                        .expect("Element should exist")
                        .try_borrow()
                        .unwrap()
                        .last_state
                }) {
                    return Some(false);
                } else {
                    return Some(true);
                }
            }
            ModuleVersion::FlipFlop => {
                if !high {
                    self.last_state = !self.last_state;
                    return Some(self.last_state);
                }
            }
            ModuleVersion::Unknown => {}
        }
        None
    }
}

fn solve(input: &[String]) -> (u64, u64) {
    let mut config = HashMap::<&str, Rc<RefCell<Module>>>::new();

    input.iter().for_each(|line| {
        let val = line
            .split(['-', '>', ',', ' '])
            .filter(|f| !f.is_empty())
            .collect_vec();
        let identifier = val.first().unwrap().to_owned();
        let (name, version) = match identifier.chars().next().unwrap() {
            'b' => (identifier, ModuleVersion::Broadcaster),
            '%' => (&identifier[1..], ModuleVersion::FlipFlop),
            '&' => (&identifier[1..], ModuleVersion::Conjonction),
            _ => (identifier, ModuleVersion::Unknown),
        };

        config.insert(
            name,
            Rc::new(RefCell::new(Module {
                version,
                inputs: Vec::new(),
                outputs: Vec::new(),
                last_state: false,
            })),
        );
    });

    input.iter().for_each(|line| {
        let split = line
            .split(['-', '>', ',', ' '])
            .filter(|f| !f.is_empty())
            .collect_vec();
        let identifier = split.first().unwrap().to_owned();
        let (name, _version) = match identifier.chars().next().unwrap() {
            'b' => (identifier, ModuleVersion::Broadcaster),
            '%' => (&identifier[1..], ModuleVersion::FlipFlop),
            '&' => (&identifier[1..], ModuleVersion::Conjonction),
            _ => (identifier, ModuleVersion::Unknown),
        };

        for output_name in &split[1..] {
            {
                let output_module = config.get_mut(output_name);

                if let Some(output_module) = output_module {
                    output_module
                        .try_borrow_mut()
                        .unwrap()
                        .inputs
                        .push(name.to_owned());
                } else {
                    println!("Adding non existing output node {}", output_name);

                    config.insert(
                        output_name,
                        Rc::new(RefCell::new(Module {
                            version: ModuleVersion::Unknown,
                            inputs: Vec::new(),
                            outputs: Vec::new(),
                            last_state: false,
                        })),
                    );
                }
            }
            {
                let this_module = config.get_mut(name).expect("Non existing module");
                this_module
                    .try_borrow_mut()
                    .unwrap()
                    .outputs
                    .push(output_name.to_string());
            }
        }
    });

    let mut count_low = 0u64;
    let mut count_high = 0u64;

    let mut queue = VecDeque::<(bool, String)>::new();
    for _ in 0..1000 {
        count_low += 1;
        queue.push_back((false, "broadcaster".to_owned()));

        while let Some((high, identifier)) = queue.pop_front() {
            let module = config.get(identifier.as_str()).unwrap();
            let signal = module.try_borrow_mut().unwrap().update(high, &config);
            if let Some(high) = signal {
                for val in module.borrow().outputs.iter() {
                    // println!("{identifier} -{} -> {}", high, &val);
                    queue.push_back((high, val.clone()));
                    if !high {
                        count_low += 1;
                    } else {
                        count_high += 1;
                    }
                }
            } else {
                // println!("{identifier} -{high}");
            }
        }
    }

    println!("{:?}", config);
    let part1 = count_low.mul(count_high);
    println!("LOW {}\tHIGH {}", count_low, count_high);
    // 644754400 is too low
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
    let input = read_lines("inputs/20.txt");
    let (part1, part2) = solve(&input);

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let input = read_lines("inputs/20_test1.txt");

        let (part1, part2) = solve(&input);
        assert_eq!(part1, 32000000);
        assert_eq!(part2, 0);
    }

    #[test]
    fn test_data2() {
        let input = read_lines("inputs/20_test2.txt");

        let (part1, part2) = solve(&input);
        assert_eq!(part1, 11687500);
        assert_eq!(part2, 0);
    }
}

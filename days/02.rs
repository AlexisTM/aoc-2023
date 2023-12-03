use std::cmp;

#[derive(Debug, PartialEq)]
struct State {
    r: usize,
    g: usize,
    b: usize,
}

impl State {
    pub fn is_possible(&self, max_state: &State) -> bool {
        self.r <= max_state.r && self.g <= max_state.g && self.b <= max_state.b
    }
}

fn parse_subgame(input: &str) -> State {
    input
        .split(',')
        .fold(State { r: 0, g: 0, b: 0 }, |acc, val| {
            let mut new_acc = acc;
            let values: Vec<&str> = val.split(' ').collect(); // ["", "3", "blue"]
            let number = values[1].parse::<usize>().unwrap();
            let color = values[2];
            if color == "red" {
                new_acc.r += number;
            } else if color == "green" {
                new_acc.g += number;
            } else if color == "blue" {
                new_acc.b += number;
            }
            new_acc
        })
}

fn main() {
    let input: Vec<String> = std::fs::read_to_string("inputs/02.txt")
        .unwrap()
        .trim_end()
        .replace("\r\n", "\n")
        .split('\n')
        .map(String::from)
        .collect();

    let max_state = State {
        r: 12,
        g: 13,
        b: 14,
    };

    let all_games: Vec<Vec<State>> = input
        .iter()
        .map(|game| {
            let val: Vec<State> = game
                .split(':')
                .last()
                .unwrap()
                .split(';') // [["1 red, 2 blue, 4 green"],..]
                .map(parse_subgame)
                .collect();
            val // [State,..]
        })
        .collect();

    let part1 = all_games
        .iter()
        .enumerate()
        .fold(0, |acc, (game_id, states)| {
            if states.iter().all(|subgame| subgame.is_possible(&max_state)) {
                acc + game_id + 1
            } else {
                acc
            }
        });
    println!("Part 1: {:?}", part1);

    let part2: usize = all_games
        .iter()
        .enumerate()
        .fold(0, |acc, (_game_id, states)| {
            let reduced = states
                .iter()
                .fold(State { r: 0, g: 0, b: 0 }, |acc, state| State {
                    r: cmp::max(acc.r, state.r),
                    g: cmp::max(acc.g, state.g),
                    b: cmp::max(acc.b, state.b),
                });

            acc + (reduced.r * reduced.b * reduced.g)
        });
    println!("Part 2: {:?}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_possible() {
        let state = State {
            r: 12,
            g: 13,
            b: 14,
        };
        assert!(state.is_possible(&State {
            r: 12,
            g: 13,
            b: 14
        }));
        assert!(state.is_possible(&State {
            r: 20,
            g: 20,
            b: 20
        }));
    }

    #[test]
    fn test_state_impossible() {
        let state = State {
            r: 10,
            g: 15,
            b: 10,
        };
        assert!(!state.is_possible(&State {
            r: 12,
            g: 13,
            b: 14
        }));
        assert!(!state.is_possible(&State { r: 0, g: 20, b: 20 }));
        assert!(!state.is_possible(&State { r: 20, g: 0, b: 20 }));
        assert!(!state.is_possible(&State { r: 20, g: 20, b: 0 }));
    }

    #[test]
    fn test_parse_subgame() {
        let state = State { r: 6, g: 9, b: 2 };

        let input = " 6 red, 9 green, 2 blue";
        assert_eq!(parse_subgame(input), state);
    }
}

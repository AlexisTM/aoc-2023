use itertools::Itertools;

#[derive(Debug, Clone)]
enum Action {
    Add,
    Remove,
}
#[derive(Debug, Clone)]
struct Instruction {
    pub id: usize,
    pub label: String,
    pub lens: Option<u16>,
    pub action: Action,
}

impl From<&str> for Instruction {
    fn from(val: &str) -> Self {
        let action;
        let mut lens = None;
        let last_char = val.chars().last().unwrap();
        if last_char.is_ascii_digit() {
            action = Action::Add;
            lens = Some((last_char as i32 - '0' as i32) as u16);
        } else {
            action = Action::Remove;
        }
        let label = val.split(['-', '=']).next().unwrap();
        Instruction {
            id: label
                .chars()
                .fold(0, |acc, c| ((acc + (c as usize)) * 17) % 256),
            label: label.to_owned(),
            lens,
            action,
        }
    }
}

fn solve(input: &[String]) -> (u64, u64) {
    let input = input.first().unwrap();
    let res = input.split(',').fold(0, |acc, s| {
        acc + s
            .chars()
            .fold(0, |sub_acc, c| ((sub_acc + (c as u64)) * 17) % 256)
    });

    let mut boxes = Vec::<Vec<Instruction>>::new();
    boxes.resize_with(256, Vec::<Instruction>::new);

    let instructions: Vec<Instruction> = input.split(',').map(|s| s.into()).collect_vec();
    for instruction in instructions.iter() {
        let i: Option<usize> = {
            let i = boxes
                .get_mut(instruction.id)
                .unwrap()
                .iter()
                .find_position(|instr| instr.label == instruction.label);

            i.map(|element| element.0)
        };
        match instruction.action {
            Action::Add => {
                if let Some(toedit) = i {
                    boxes[instruction.id][toedit].lens = instruction.lens;
                } else {
                    boxes[instruction.id].push((*instruction).clone());
                }
            }
            Action::Remove => {
                if let Some(toremove) = i {
                    boxes[instruction.id].remove(toremove);
                }
            }
        }
    }

    let mut part2 = 0;
    for (i, curr) in boxes.iter().enumerate() {
        if !curr.is_empty() {
            for (j, lens) in curr.iter().enumerate() {
                part2 += (i + 1) * (j + 1) * lens.lens.unwrap() as usize;
            }
            // println!("Box {}", i);
            // println!("{:?}", curr);
        }
    }

    // [0-255] (256 boxes, multiple lens slots)
    // [1 - 9] (8 lenses)
    // sequence label the step operates
    // Operation: = or -
    // - means: remove go to the box with label, and remove that labelled lens from the box
    // = means: Add the lens with label to the box
    //     - If already a lens in the box with the same label, replace it
    //     - Else, push_back the lens (all the way to the front?, may need a reverse at the end)
    let part1 = res;
    (part1, part2 as u64)
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
    let input = read_lines("inputs/15.txt");
    let (part1, part2) = solve(&input);
    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let input = read_lines("inputs/15_test.txt");

        let (part1, part2) = solve(&input);
        assert_eq!(part1, 1320);
        assert_eq!(part2, 0);
    }
}

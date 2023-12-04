fn solve(input: &[String]) -> (u64, u64) {
    let data_input: Vec<u64> = input
        .iter()
        .map(|card| {
            let split: Vec<&str> = card.split(':').last().unwrap().split('|').collect();
            let good_cards: Vec<&str> = split[0].split(' ').filter(|val| !val.is_empty()).collect();
            let my_cards: Vec<&str> = split[1].split(' ').filter(|val| !val.is_empty()).collect();
            (good_cards, my_cards)
        })
        .map(|(good_cards, my_cards)| {
            let mut val = 0;
            for card in my_cards {
                if good_cards.contains(&card) {
                    val += 1;
                }
            }
            val
        })
        .collect();

    let part1 = data_input.iter().fold(0u64, |acc, val| {
        if *val >= 1 {
            return acc + (1 << (*val - 1));
        }
        acc
    });

    let mut counts = Vec::<u64>::new();
    counts.resize(data_input.len(), 1);

    for (index, card) in data_input.iter().enumerate() {
        let count = counts[index];
        for i in (index + 1)..(index + 1 + (*card as usize)) {
            if i < counts.len() {
                counts[i] += count;
            }
        }
    }

    let part2 = counts.iter().sum::<u64>();
    (part1, part2)
}

fn main() {
    let input: Vec<String> = std::fs::read_to_string("inputs/04.txt")
        .unwrap()
        .trim_end()
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
        let input: Vec<String> = std::fs::read_to_string("inputs/04_test.txt")
            .unwrap()
            .trim_end()
            .replace("\r\n", "\n") // Windows duh
            .split('\n')
            .map(String::from)
            .collect();

        let (part1, part2) = solve(&input);
        assert_eq!(part1, 13);
        assert_eq!(part2, 30);
    }
}

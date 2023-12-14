use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    StrongestCard = 0,
    Pair,
    TwoPairs,
    Triple,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    pub kind: HandKind,
    pub hand: [u8; 5],
    pub bid: u64,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct HandP2 {
    pub kind: HandKind,
    pub hand: [u8; 5],
    pub bid: u64,
}

fn hand_kind(hand: &[u8; 5]) -> HandKind {
    let mut counts = [0; 13];
    for card in hand.iter() {
        counts[(card % 13) as usize] += 1;
    }
    counts.sort_unstable();
    let max_count = *counts.last().unwrap();
    let second_max_count = *counts.iter().rev().nth(1).unwrap();
    match (max_count, second_max_count) {
        (5, _) => HandKind::FiveOfAKind,
        (4, _) => HandKind::FourOfAKind,
        (3, 2) => HandKind::FullHouse,
        (3, _) => HandKind::Triple,
        (2, 2) => HandKind::TwoPairs,
        (2, _) => HandKind::Pair,
        (1, _) => HandKind::StrongestCard,
        _ => panic!("Invalid hand"),
    }
}

impl Hand {
    pub fn new(input: &str) -> Self {
        let val = input.split(' ').collect_vec();
        let hand: [u8; 5] = val[0]
            .chars()
            .map(|c| match c {
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => 0,
            })
            .collect_vec()
            .try_into()
            .unwrap();

        let bid: u64 = val[1].parse().unwrap();
        Hand {
            kind: hand_kind(&hand),
            bid,
            hand,
        }
    }
}

fn hand_kind_p2(hand: &[u8; 5]) -> HandKind {
    let mut counts = [0; 13];
    for card in hand.iter() {
        counts[(card % 13) as usize] += 1;
    }
    let jokers = counts[0];
    let mut other_cards_counts = counts[1..13].to_owned();
    other_cards_counts.sort_unstable();

    let max_count = *other_cards_counts.last().unwrap();
    let second_max_count = *other_cards_counts.iter().rev().nth(1).unwrap();
    match (max_count + jokers, second_max_count) {
        (5, _) => HandKind::FiveOfAKind,
        (4, _) => HandKind::FourOfAKind,
        (3, 2) => HandKind::FullHouse,
        (3, _) => HandKind::Triple,
        (2, 2) => HandKind::TwoPairs,
        (2, _) => HandKind::Pair,
        (1, _) => HandKind::StrongestCard,
        _ => panic!("Invalid hand"),
    }
}

impl HandP2 {
    pub fn new(input: &str) -> Self {
        let val = input.split(' ').collect_vec();
        let hand: [u8; 5] = val[0]
            .chars()
            .map(|c| match c {
                'J' => 0, // J is the joker
                '2' => 1,
                '3' => 2,
                '4' => 3,
                '5' => 4,
                '6' => 5,
                '7' => 6,
                '8' => 7,
                '9' => 8,
                'T' => 9,
                'Q' => 10,
                'K' => 11,
                'A' => 12,
                _ => 0,
            })
            .collect_vec()
            .try_into()
            .unwrap();

        let bid: u64 = val[1].parse().unwrap();
        HandP2 {
            kind: hand_kind_p2(&hand),
            bid,
            hand,
        }
    }
}

fn solve(input: &[String]) -> (u64, u64) {
    // Exactly one type:
    // Five of a kind
    // Four of a kind
    // Full house (3+2)
    // Triple
    // Two pairs
    // Pairs
    // Strongest card

    // The card value for which we got the strength doesn't matter.
    // Winning on draw: Compare first card => Bigger = win

    // Hand followed by bid

    // Step 1: Evaluate the strength of each Hand
    let mut hands: Vec<Hand> = input.iter().map(|s| Hand::new(s)).collect_vec();
    // Step 2: Sort the hands from weakest to strongest
    hands.sort_unstable();
    // Step 3: Winnings = bid * position_in_sorted_list
    let part1 = hands.iter().enumerate().fold(0u64, |acc, (index, hand)| {
        acc + (index as u64 + 1u64) * hand.bid
    });

    // Part 2, identical, but J is the smallest card, and counts towards the max count.
    // This changes the way the input data is, and couldn't be nicely integrated into the current
    // solution. :(
    let mut hands: Vec<HandP2> = input.iter().map(|s| HandP2::new(s)).collect_vec();
    hands.sort_unstable();
    let part2 = hands.iter().enumerate().fold(0u64, |acc, (index, hand)| {
        acc + (index as u64 + 1u64) * hand.bid
    });
    (part1, part2)
}

fn main() {
    let input: Vec<String> = std::fs::read_to_string("inputs/07.txt")
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
        let input: Vec<String> = std::fs::read_to_string("inputs/07_test.txt")
            .unwrap()
            .trim_end()
            .replace("\r\n", "\n") // Windows duh
            .split('\n')
            .map(String::from)
            .collect();

        let (part1, part2) = solve(&input);
        assert_eq!(part1, 6440);
        assert_eq!(part2, 5905);
    }

    #[test]
    fn test_hand_kind() {
        assert_eq!(hand_kind(&[3, 2, 10, 3, 13]), HandKind::Pair);
    }
}

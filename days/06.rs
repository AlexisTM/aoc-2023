use itertools::Itertools;


fn would_win(time_pressed: i64, distance: i64, max_time: i64) -> bool {
    time_pressed * (max_time - time_pressed) > distance
}

fn solve(input: &[String]) -> (i64, i64) {
    let times: Vec<i64> = input.first().unwrap().split(':').last().unwrap().split(' ').filter(|s| !s.is_empty()).map(|n|n.parse().unwrap()).collect_vec();
    let distances: Vec<i64> = input.last().unwrap().split(':').last().unwrap().split(' ').filter(|s| !s.is_empty()).map(|n|n.parse().unwrap()).collect_vec();
    println!("{:?} {:?}", times, distances);


    let part1 = times.iter().zip(distances).map(|(time, distance)| {
        let mut result = 0;
        for t in 0..*time {
            if would_win(t, distance, *time) {
                result += 1;
            }
        }
        result
    }).product::<i64>();
    let part2 = {
        let time =  44707080;
        let distance = 283113411341491;

        (0..time).fold(0, |acc, t| {
            if would_win(t, distance, time) {
                return acc + 1;
            }
            acc
        })
    };
    (part1, part2)
}

fn main() {
    let input: Vec<String> = std::fs::read_to_string("inputs/06.txt")
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
        let input: Vec<String> = std::fs::read_to_string("inputs/06_test.txt")
            .unwrap()
            .trim_end()
            .replace("\r\n", "\n") // Windows duh
            .split('\n')
            .map(String::from)
            .collect();

        let (part1, part2) = solve(&input);
        assert_eq!(part1, 288);
        assert_eq!(part2, 29432455);
    }
}

use itertools::Itertools;

fn is_mirrored(mirror: i64, game: &[Vec<char>], limit: u64) -> bool {
    let mut diff = 0;
    for checked_value in 0..(mirror + 1) {
        let left_index = mirror - (checked_value);
        let right_index = mirror + (checked_value + 1);

        if left_index < 0 || right_index >= game.len() as i64 {
            break;
        }

        diff += game[left_index as usize]
            .iter()
            .zip(&game[right_index as usize])
            .fold(0, |acc, (a, b)| if a != b { acc + 1 } else { acc });
    }
    diff == limit
}

fn find_mirrors(game: &[Vec<char>]) -> ((u64, u64), (u64, u64)) {
    let mut row_mirror = 0u64;
    let mut col_mirror = 0u64;
    let mut row_mirror_p2 = 0u64;
    let mut col_mirror_p2 = 0u64;

    let row_count = game.len();
    for mirror in 0..(row_count - 1) {
        if is_mirrored(mirror as i64, game, 0) {
            row_mirror = (mirror + 1) as u64;
        }
        if is_mirrored(mirror as i64, game, 1) {
            row_mirror_p2 = (mirror + 1) as u64;
        }
    }
    let transposed_game: Vec<Vec<char>> = (0..game[0].len())
        .map(|col| game.iter().map(|row| row[col]).collect())
        .collect();

    let col_count = transposed_game.len();
    for mirror in 0..(col_count - 1) {
        if is_mirrored(mirror as i64, &transposed_game, 0) {
            col_mirror = (mirror + 1) as u64;
        }
        if is_mirrored(mirror as i64, &transposed_game, 1) {
            col_mirror_p2 = (mirror + 1) as u64;
        }
    }

    ((row_mirror, col_mirror), (row_mirror_p2, col_mirror_p2))
}

fn solve(input: &[String]) -> (u64, u64) {
    let mut games = Vec::<Vec<Vec<char>>>::new();
    games.push(Vec::<Vec<char>>::new());
    input.iter().for_each(|f| {
        if f.is_empty() {
            games.push(Vec::<Vec<char>>::new());
        } else {
            games.last_mut().unwrap().push(f.chars().collect_vec());
        }
    });

    let mut part1 = 0;
    let mut part2 = 0;
    for game in games.iter() {
        let ((row_mirror, col_mirror), (row_mirror_p2, col_mirror_p2)) = find_mirrors(game);
        part1 += col_mirror + row_mirror * 100u64;
        part2 += col_mirror_p2 + row_mirror_p2 * 100u64;
    }

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
    let input = read_lines("inputs/13.txt");
    let (part1, part2) = solve(&input);
    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let input = read_lines("inputs/13_test.txt");

        let (part1, part2) = solve(&input);
        assert_eq!(part1, 405);
        assert_eq!(part2, 0);
    }
}

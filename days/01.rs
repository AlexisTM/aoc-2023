fn find_digits(input: &str) -> i32 {
    let mut first = 'a';
    let mut last = 'a';

    input.chars().filter(|c| c.is_numeric()).for_each(|c| {
        if first == 'a' {
            first = c;
        } else {
            last = c;
        }
    });

    if last == 'a' {
        last = first
    }

    let mut integer_str = String::from(first);
    integer_str.push(last);
    integer_str.parse::<i32>().unwrap()
}

fn replace_text_by_digits(input: &str) -> String {
    input
        //.replace("zero", "0")
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
}

fn main() {
    let input: Vec<String> = std::fs::read_to_string("inputs/01.txt")
        .unwrap()
        .trim_end()
        .split('\n')
        .map(String::from)
        .collect();

    let result = input.iter().fold(0, |acc, s| acc + find_digits(s));
    println!("Part 1: {:?}", result);

    let result = input
        .iter()
        .map(|s| replace_text_by_digits(s))
        .fold(0, |acc, s| acc + find_digits(&s));
    println!("Part 2: {:?}", result);
}

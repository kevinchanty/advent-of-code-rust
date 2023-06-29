use std::fs;

const INPUT_PATH: &str = "./input.txt";

fn get_score_for_alphabet(input: &char) -> usize {
    let alphabet: Vec<char> = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();

    alphabet
        .iter()
        .position(|alphabet| alphabet == input)
        .expect("Can't get score for non-alphabet")
}

fn get_score_for_duplicated(input: Vec<char>) -> usize {
    input.iter().map(|char| get_score_for_alphabet(char)).sum()
}

fn find_duplicated(input: &str) -> Vec<char> {
    let mid_index = match input.len() % 2 {
        0 => input.len() / 2,
        _ => (input.len() - 1) / 2,
    };

    let (first_half, second_half) = input.split_at(mid_index);

    println!(
        "Input: {}, First half:{}, second half: {}",
        input, first_half, second_half
    );

    let mut duplicated: Vec<char> = vec![];

    first_half.chars().for_each(|char| {
        if second_half.contains(char) {
            duplicated.push(char);
        }
    });

    duplicated.sort();
    duplicated.dedup();
    println!("Duplicated: {:?}", duplicated);

    duplicated
}

fn part1() {
    let input_string: String = fs::read_to_string(INPUT_PATH).unwrap();
    let haha: usize = input_string
        .lines()
        .map(|x| find_duplicated(x))
        .map(|duplicated| get_score_for_duplicated(duplicated))
        .sum();

    println!("{}", haha);
}

fn main() {
    println!("Part 1:");
    part1();
}

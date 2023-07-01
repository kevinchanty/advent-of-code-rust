use std::fs;

const INPUT_PATH: &str = "./input.txt";

fn calculate_score(input: &char) -> usize {
    "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .position(|x| &x == input)
        .expect("Calculate Score received non alphabet!")
}

fn get_score_for_duplicated(input: Vec<char>) -> usize {
    input.iter().map(|char| calculate_score(char)).sum()
}

fn find_duplicated(input: &str) -> Vec<char> {
    let mid_index = match input.len() % 2 {
        0 => input.len() / 2,
        _ => (input.len() - 1) / 2,
    };

    let (first_half, second_half) = input.split_at(mid_index);

    let mut duplicated: Vec<char> = vec![];

    first_half.chars().for_each(|char| {
        if second_half.contains(char) {
            duplicated.push(char);
        }
    });

    duplicated.sort();
    duplicated.dedup();

    duplicated
}

fn part1() {
    let input_string: String = fs::read_to_string(INPUT_PATH).unwrap();
    let part_1_answer: usize = input_string
        .lines()
        .map(|x| find_duplicated(x))
        .map(|duplicated| get_score_for_duplicated(duplicated))
        .sum();

    println!("{}", part_1_answer);
}

fn part2() {
    let answer: usize = fs::read_to_string("./input.txt")
        .expect("No such files")
        .lines()
        .enumerate()
        // group by 3
        .fold(Vec::new(), |mut acc: Vec<Vec<&str>>, (index, input)| {
            if index % 3 == 0 {
                acc.push(vec![input]);
            } else {
                acc.last_mut().unwrap().push(input);
            }
            acc
        })
        .iter()
        // find badge which is in all 3 backpack
        .map(|input| {
            input[0]
                .chars()
                .find(|char| input[1].contains(*char) && input[2].contains(*char))
                .expect("No badge found in a group!")
        })
        .map(|x| calculate_score(&x))
        .sum();

    println!("{}", answer)
}

fn main() {
    println!("Part 1:");
    part1();
    println!("Part 2 : ");
    part2();
}

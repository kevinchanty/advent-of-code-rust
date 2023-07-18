const INIT_STATE_INPUT: &str = include_str!("../init_input.txt");
const INPUT: &str = include_str!("../input.txt");

fn get_init_state() -> Vec<Vec<char>> {
    let max_crate = INIT_STATE_INPUT
        .lines()
        .last()
        .unwrap()
        .split("  ")
        .last()
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap();

    let mut result: Vec<Vec<char>> = vec![vec![]; max_crate];

    INIT_STATE_INPUT.lines().rev().skip(1).for_each(|x| {
        x.chars().enumerate().for_each(|(i, char)| {
            if char.is_alphabetic() {
                result[(i - 1) / 4].push(char)
            }
        })
    });

    return result;
}

fn parse_operation(input: &str) -> (i32, i32, i32) {
    (0, 0, 0)
}

fn part_one() {
    let answer = "haha";
    println!("{}", answer);
    let state = get_init_state();

    INPUT.lines().map(|x| parse_operation())
}

fn main() {
    println!("Part one:");
    part_one();
}

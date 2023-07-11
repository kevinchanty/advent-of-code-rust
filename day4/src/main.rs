const INPUT: &str = include_str!("../input.txt");

fn parse_input(input: &str) -> (i32, i32, i32, i32) {
    let splitted: Vec<i32> = input
        .split(['-', ','])
        .map(|i| i.parse::<i32>().unwrap())
        .collect();

    (splitted[0], splitted[1], splitted[2], splitted[3])
}

fn part_one() {
    let answer = INPUT
        .lines()
        .map(|line| parse_input(line))
        .filter(|(first_low, first_high, second_low, second_high)| {
            (first_low >= second_low && first_high <= second_high)
                || (first_low <= second_low && first_high >= second_high)
        })
        .count();

    println!("{answer}");
}

fn part_two() {
    let answer = INPUT
        .lines()
        .map(|line| parse_input(line))
        .filter(|(first_low, first_high, second_low, second_high)| {
            (first_low <= second_high && first_low >= second_low)
                || (second_low <= first_high && second_low >= first_low)
        })
        .count();

    println!("{answer}");
}

fn main() {
    println!("Part 1:");
    part_one();
    println!("Part 2:");
    part_two();
}

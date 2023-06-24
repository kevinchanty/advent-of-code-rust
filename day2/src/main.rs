const INPUT_STRING: &str = include_str!("../input.txt");

fn calculate_score(choice_string: &str) -> u32 {
    let mut score: u32 = 0;
    let mut choice_iter = choice_string.split(" ");
    let opponent = choice_iter.next().unwrap();
    let mine = choice_iter.next().unwrap();

    // Base Score
    match mine {
        "X" => score = score + 1,
        "Y" => score = score + 2,
        "Z" => score = score + 3,
        _ => (),
    }

    // Outcome Score
    match mine {
        "X" => match opponent {
            "A" => score = score + 3,
            "B" => score = score + 0,
            "C" => score = score + 6,
            _ => (),
        },
        "Y" => match opponent {
            "A" => score = score + 6,
            "B" => score = score + 3,
            "C" => score = score + 0,
            _ => (),
        },
        "Z" => match opponent {
            "A" => score = score + 0,
            "B" => score = score + 6,
            "C" => score = score + 3,
            _ => (),
        },
        _ => (),
    }

    score
}

fn part1() {
    println!("Part 1: ");
    println!(
        "{}",
        INPUT_STRING
            .lines()
            .map(|round_string| calculate_score(round_string))
            .sum::<u32>()
    )
}

fn calculate_score_2(choice_string: &str) -> u32 {
    let mut score: u32 = 0;
    let mut choice_iter = choice_string.split(" ");
    let opponent = choice_iter.next().unwrap();
    let instruction = choice_iter.next().unwrap();

    // Outcome Score
    match instruction {
        "X" => score = score + 0,
        "Y" => score = score + 3,
        "Z" => score = score + 6,
        _ => panic!("Bad!"),
    }

    // Base Score
    match instruction {
        "X" => match opponent {
            "A" => score = score + 3,
            "B" => score = score + 1,
            "C" => score = score + 2,
            _ => (),
        },
        "Y" => match opponent {
            "A" => score = score + 1,
            "B" => score = score + 2,
            "C" => score = score + 3,
            _ => (),
        },
        "Z" => match opponent {
            "A" => score = score + 2,
            "B" => score = score + 3,
            "C" => score = score + 1,
            _ => (),
        },
        _ => (),
    }

    score
}

fn part2() {
    println!("Part 2: ");
    println!(
        "{}",
        INPUT_STRING
            .lines()
            .map(|round_input| calculate_score_2(round_input))
            .sum::<u32>()
    )
}
fn main() {
    part1();
    part2();
}

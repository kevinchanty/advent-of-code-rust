const INPUT: &str = std::include_str!("../input.txt");

fn part1() {
    println!("Part 1");

    let mut max_calories: u32 = 0;
    let mut current_count: u32 = 0;

    for calories_string in INPUT.split("\r\n") {
        match calories_string.parse::<u32>() {
            Ok(calories) => current_count = current_count + calories,
            Err(_) => {
                if current_count > max_calories {
                    max_calories = current_count;
                }
                current_count = 0;
            }
        }
    }

    println!("{max_calories}");
}

fn part2() {
    println!("Part 2");

    let mut top_three: Vec<u32> = vec![];
    let mut current_count: u32 = 0;

    for calories_string in INPUT.split("\r\n") {
        match calories_string.parse::<u32>() {
            Ok(calories) => current_count = current_count + calories,
            Err(_) => {
                if top_three.len() < 3 {
                    top_three.push(current_count);
                } else if current_count > *top_three.last().unwrap() {
                    top_three[1] = current_count;
                }
                top_three.sort();
                current_count = 0;
            }
        }
    }

    print!("{}", top_three.iter().sum::<u32>());
}

fn main() {
    part1();
    part2();
}

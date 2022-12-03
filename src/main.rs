use std::env;

mod days;
mod util;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let day = arguments[1].as_str();
    let part = arguments[2].as_str();

    match day {
        "1" => match part {
            "1" => days::one::part1(),
            "2" => days::one::part2(),
            &_ => println!("Can't find part."),
        },
        "2" => match part {
            "1" => days::two::part1(),
            "2" => days::two::part2(),
            &_ => println!("Can't find part."),
        },
        "3" => match part {
            "1" => days::three::part1(),
            "2" => days::three::part2(),
            &_ => println!("Can't find part."),
        },
        &_ => println!("Can't find day"),
    }
}

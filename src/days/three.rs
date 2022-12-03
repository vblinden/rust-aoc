pub fn part1() {
    let input = crate::util::input::file("input/day3");
    let bags = input.split("\n").filter(|item| !item.is_empty());

    let mut total = 0;

    for bag in bags {
        let compartment_size = bag.len() / 2;
        let items: Vec<&str> = bag.split("").filter(|s| !s.is_empty()).collect();

        let mut left_compartment: Vec<&str> = items[0..compartment_size].iter().cloned().collect();
        left_compartment.sort();
        left_compartment.dedup();

        let right_compartment: Vec<&str> = items[compartment_size..].iter().cloned().collect();

        for item in left_compartment {
            if right_compartment.contains(&item) {
                total += priority(item);
            }
        }
    }

    println!("{total}")
}

pub fn part2() {
    let input = crate::util::input::file("input/day3");
    let bags: Vec<&str> = input
        .split("\n")
        .filter(|item| !item.is_empty())
        .into_iter()
        .collect();

    let mut total = 0;

    for i in (0..bags.len()).step_by(3) {
        for item in bags[i].chars() {
            if bags[i + 1].contains(item) && bags[i + 2].contains(item) {
                total += priority(item.to_string().as_str());
                break;
            }
        }
    }

    println!("{total}");
}

fn priority(char: &str) -> i32 {
    match char {
        "a" => return 1,
        "b" => return 2,
        "c" => return 3,
        "d" => return 4,
        "e" => return 5,
        "f" => return 6,
        "g" => return 7,
        "h" => return 8,
        "i" => return 9,
        "j" => return 10,
        "k" => return 11,
        "l" => return 12,
        "m" => return 13,
        "n" => return 14,
        "o" => return 15,
        "p" => return 16,
        "q" => return 17,
        "r" => return 18,
        "s" => return 19,
        "t" => return 20,
        "u" => return 21,
        "v" => return 22,
        "w" => return 23,
        "x" => return 24,
        "y" => return 25,
        "z" => return 26,
        "A" => return 27,
        "B" => return 28,
        "C" => return 29,
        "D" => return 30,
        "E" => return 31,
        "F" => return 32,
        "G" => return 33,
        "H" => return 34,
        "I" => return 35,
        "J" => return 36,
        "K" => return 37,
        "L" => return 38,
        "M" => return 39,
        "N" => return 40,
        "O" => return 41,
        "P" => return 42,
        "Q" => return 43,
        "R" => return 44,
        "S" => return 45,
        "T" => return 46,
        "U" => return 47,
        "V" => return 48,
        "W" => return 49,
        "X" => return 50,
        "Y" => return 51,
        "Z" => return 52,
        &_ => return 0,
    }
}

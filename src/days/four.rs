pub fn part1() {
    let input = crate::util::input::file("input/day4.txt");
    let pairs = input.lines().map(|pair| {
        let split: Vec<&str> = pair.split(",").collect();
        let first: Vec<i32> = extract_pair(split[0]);
        let second: Vec<i32> = extract_pair(split[1]);

        return vec![
            Pair {
                start: first[0],
                end: first[1],
            },
            Pair {
                start: second[0],
                end: second[1],
            },
        ];
    });

    let mut total = 0;

    for pair in pairs {
        let first = &pair[0];
        let second = &pair[1];

        if within(first, second) || within(second, first) {
            total += 1;
        }
    }

    println!("Total pairs: {total}");
}

pub fn part2() {
    let input = crate::util::input::file("input/day4.txt");
    let pairs = input.lines().map(|pair| {
        let split: Vec<&str> = pair.split(",").collect();
        let first: Vec<i32> = extract_pair(split[0]);
        let second: Vec<i32> = extract_pair(split[1]);

        return vec![
            Pair {
                start: first[0],
                end: first[1],
            },
            Pair {
                start: second[0],
                end: second[1],
            },
        ];
    });

    let mut total = 0;

    for pair in pairs {
        let first = &pair[0];
        let second = &pair[1];

        if first.start.max(second.start) <= first.end.min(second.end) {
            total += 1;
        }
    }

    println!("Total pairs: {total}");
}

struct Pair {
    start: i32,
    end: i32,
}

fn within(first: &Pair, second: &Pair) -> bool {
    first.start >= second.start && first.end <= second.end
}

fn extract_pair(input: &str) -> Vec<i32> {
    return input
        .split("-")
        .map(|i| i.parse::<i32>().unwrap())
        .collect();
}

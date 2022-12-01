pub fn part1() {
    let input = crate::utilities::input::string("input/day1/part1.txt");
    let calories = input.split("\n\n");

    let mut highest_calories = 0;

    calories.for_each(|cal| {
        let total_calories: i32 = cal
            .split("\n")
            .filter(|x| !x.is_empty())
            .map(|cal| cal.parse().unwrap_or(0))
            .sum();

        if total_calories >= highest_calories {
            highest_calories = total_calories;
        }
    });

    println!("{highest_calories}");
}

pub fn part2() {
    let input = crate::utilities::input::string("input/day1/part1.txt");
    let calories = input.split("\n\n");

    let mut calories_array: Vec<i32> = Vec::new();

    calories.for_each(|cal| {
        let total_calories: i32 = cal
            .split("\n")
            .filter(|x| !x.is_empty())
            .map(|cal| cal.parse().unwrap_or(0))
            .sum();

        calories_array.push(total_calories);
    });

    calories_array.sort();
    calories_array.reverse();

    let reserve_calories = calories_array[0] + calories_array[1] + calories_array[2];

    println!("{reserve_calories}");
}

pub fn part1() {
    let input = crate::util::input::file("input/day2");
    let games = input.split("\n").filter(|string| !string.is_empty());

    let mut total_opponent_score = 0;
    let mut total_player_score = 0;

    for game in games {
        let choices: Vec<&str> = game.split(" ").collect();
        let opponent_choice = choices[0];
        let player_choice = choices[1];
        let score = calculate_game_score(opponent_choice, player_choice);

        total_opponent_score += score.opponent;
        total_player_score += score.player;
    }

    println!("{total_opponent_score} - {total_player_score}")
}

pub fn part2() {
    let input = crate::util::input::file("input/day2");
    let games = input.split("\n").filter(|string| !string.is_empty());

    let mut total_opponent_score = 0;
    let mut total_player_score = 0;

    for game in games {
        let choices: Vec<&str> = game.split(" ").collect();
        let opponent_choice = choices[0];
        let preferred_outcome = choices[1];
        let player_choice = choice_based_on_preferred_outcome(opponent_choice, preferred_outcome);

        let score = calculate_game_score(opponent_choice, player_choice);

        total_opponent_score += score.opponent;
        total_player_score += score.player;
    }

    println!("{total_opponent_score} - {total_player_score}")
}

fn points_for_choice(choice: &str) -> i32 {
    let mut score = 0;

    match choice {
        "A" => score += 1,
        "B" => score += 2,
        "C" => score += 3,
        "X" => score += 1,
        "Y" => score += 2,
        "Z" => score += 3,
        &_ => println!("Invalid choice"),
    }

    return score;
}

struct Result {
    opponent: i32,
    player: i32,
}

fn calculate_game_score(opponent_choice: &str, player_choice: &str) -> Result {
    let mut opponent_score = points_for_choice(opponent_choice);
    let mut player_score = points_for_choice(player_choice);

    // Opponent wins...
    if (opponent_choice == "A" && player_choice == "Z")
        || (opponent_choice == "C" && player_choice == "Y")
        || (opponent_choice == "B" && player_choice == "X")
    {
        opponent_score += 6;
    }
    // Player wins...
    else if (player_choice == "X" && opponent_choice == "C")
        || (player_choice == "Z" && opponent_choice == "B")
        || (player_choice == "Y" && opponent_choice == "A")
    {
        player_score += 6;
    }
    // Draw...
    else {
        opponent_score += 3;
        player_score += 3;
    }

    return Result {
        player: player_score,
        opponent: opponent_score,
    };
}

fn choice_based_on_preferred_outcome<'a>(
    opponent_choice: &'a str,
    preferred_outcome: &'a str,
) -> &'a str {
    // We need a draw...
    if preferred_outcome == "Y" {
        return opponent_choice;
    }

    // We need to lose...
    if preferred_outcome == "X" {
        match opponent_choice {
            "A" => return "Z",
            "B" => return "X",
            "C" => return "Y",
            &_ => return "",
        }
    }

    // We need to win...
    if preferred_outcome == "Z" {
        match opponent_choice {
            "A" => return "Y",
            "B" => return "Z",
            "C" => return "X",
            &_ => return "",
        }
    }

    return "A";
}

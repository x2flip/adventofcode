use std::fs;

#[derive(Debug)]
struct GameTotals {
    game: i32,
    green: i32,
    red: i32,
    blue: i32,
    power: i32,
}

fn main() {
    let mut game_totals: Vec<GameTotals> = vec![];
    let example = fs::read_to_string("content.txt").expect("Could not read the example file");
    let _ = example.lines().for_each(|row| {
        let mut game_and_totals = row.split(": ");
        let game_number = game_and_totals
            .next()
            .unwrap()
            .split(" ")
            .nth(1)
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let totals = game_and_totals.next().unwrap();
        let mut green = 0;
        let mut red = 0;
        let mut blue = 0;

        totals.split("; ").for_each(|game| {
            game.split(", ").for_each(|draw| {
                let qty = draw
                    .split(" ")
                    .nth(0)
                    .unwrap()
                    .to_string()
                    .parse::<i32>()
                    .unwrap();

                if draw.contains("red") {
                    if qty > red {
                        red = qty;
                    }
                }
                if draw.contains("blue") {
                    if qty > blue {
                        blue = qty
                    }
                }
                if draw.contains("green") {
                    if qty > green {
                        green = qty
                    }
                }
            })
        });

        let power = green * red * blue;

        game_totals.push(GameTotals {
            game: game_number,
            green,
            red,
            blue,
            power,
        })
    });
    let mut possible_games: Vec<i32> = vec![];
    let mut sum_of_power = 0;
    game_totals.iter().for_each(|item| {
        sum_of_power += item.power;
        if check_if_possible(item) {
            println!("This game is possible: {:?}", item);
            possible_games.push(item.game)
        };
    });

    println!("{:?}", possible_games);

    let mut sum = 0;
    possible_games.iter().for_each(|i| sum += i);
    println!("{}", sum);
    println!("{:?}", sum_of_power);
}

fn check_if_possible(game: &GameTotals) -> bool {
    let red_count = 12;
    let blue_count = 14;
    let green_count = 13;

    if game.red > red_count || game.blue > blue_count || game.green > green_count {
        false
    } else {
        true
    }
}

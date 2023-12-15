use std::fs;

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}
impl Race {
    fn new(time: i64, distance: i64) -> Race {
        Race { time, distance }
    }
}

fn main() {
    let example = "Time:      7  15   30
Distance:  9  40  200"
        .to_string();

    let input = fs::read_to_string("input.txt").expect("Error reading file");

    let races = get_races(&input);

    let mut ways_to_win = vec![];

    races
        .iter()
        .for_each(|i| ways_to_win.push(get_ways_to_win(i)));

    println!("Races: {:#?}", races);
    println!("Ways to Win: {:#?}", ways_to_win);

    let mut res = 1;

    ways_to_win.iter().for_each(|i| {
        res *= i;
    });

    println!("Result: {:#?}", res);

    let p2_race = get_race_p2(&input);

    let mut ways_to_win_p2 = get_ways_to_win(&p2_race);

    println!("Part 2 Ways to Win: {:#?}", ways_to_win_p2);
}

fn get_ways_to_win(race: &Race) -> i64 {
    let mut result = 0;
    for i in 0..race.time + 1 {
        let button_hold = i;
        let remaining_time = race.time - i;
        let dist_traveled = i * remaining_time;
        println!(
            "Hold the button for {:#?} milliseconds. Then, the boat will have travelled {:#?} millimeters in {:#?} milliseconds",
            button_hold, dist_traveled, remaining_time
        );

        if dist_traveled > race.distance {
            println!("Button Hold Added: {:#?}", button_hold);
            result += 1;
        }
    }

    result
}

fn get_races(string: &str) -> Vec<Race> {
    let mut spl = string.split("\n");
    let times = spl.next().unwrap();
    let dist = spl.next().unwrap();
    let mut times_vec = vec![];
    let mut dist_vec = vec![];

    times.split(": ").last().unwrap().split(" ").for_each(|c| {
        if c == " " || c == "" {
            return;
        } else {
            times_vec.push(c.to_string().parse::<i64>().unwrap());
        }
    });

    dist.split(": ").last().unwrap().split(" ").for_each(|c| {
        if c == " " || c == "" {
            return;
        } else {
            dist_vec.push(c.to_string().parse::<i64>().unwrap());
        }
    });

    times_vec
        .into_iter()
        .zip(dist_vec.into_iter())
        .map(|(time, dist)| Race::new(time, dist))
        .collect()
}

fn get_race_p2(string: &str) -> Race {
    let mut spl = string.split("\n");
    let times = spl.next().unwrap();
    let dist = spl.next().unwrap();
    let mut times_vec = vec![];
    let mut dist_vec = vec![];

    times.split(": ").last().unwrap().split(" ").for_each(|c| {
        if c == " " || c == "" {
            return;
        } else {
            times_vec.push(c.to_string());
        }
    });

    dist.split(": ").last().unwrap().split(" ").for_each(|c| {
        if c == " " || c == "" {
            return;
        } else {
            dist_vec.push(c.to_string());
        }
    });

    let conc_time = times_vec.join("").parse::<i64>().unwrap();
    let conc_dist = dist_vec.join("").parse::<i64>().unwrap();

    Race {
        time: conc_time,
        distance: conc_dist,
    }
}

use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Res {
    id: usize,
    matches: i32,
    copies: i32,
}

fn main() {
    let test_card = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
        .to_string();

    let input = fs::read_to_string("input.txt").expect("Error reading input file");

    let mut scratch_card = vec![];
    let mut result: Vec<Res> = Vec::new();

    input.lines().enumerate().for_each(|(index, row)| {
        let mut card = vec![];
        let mut spl = row.split(" | ");
        let mut lucky = vec![];
        spl.nth(0)
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .split(" ")
            .for_each(|num| {
                if num != "" {
                    lucky.push(num.to_string().parse::<i32>().unwrap())
                }
            });

        let mut scratch = vec![];
        spl.last().unwrap().split(" ").for_each(|num| {
            if num != "" {
                scratch.push(num.to_string().parse::<i32>().unwrap())
            }
        });
        card.push(lucky);
        card.push(scratch);
        scratch_card.push(card);

        let mut res = Res {
            id: (index as i32 + 1) as usize,
            matches: 0,
            copies: 1,
        };
        result.push(res);
    });

    let mut winning_points = vec![];

    result
        .iter()
        .for_each(|card| println!("Scratch Card: {:?}", card));

    scratch_card.iter().enumerate().for_each(|(index, card)| {
        println!("Card: {:?}", card);
        let lucky = card.first().unwrap();
        let scratch = card.last().unwrap();
        let mut winning_point = 0;
        let mut matches = 0;
        let res_card = result.get(index).unwrap();
        let copies = res_card.copies;
        println!("result card: {:?}", res_card);
        println!("copies : {:?}", copies);

        lucky.iter().for_each(|lucky_num| {
            scratch.iter().for_each(|scratched| {
                if lucky_num == scratched {
                    matches += 1;
                    if winning_point == 0 {
                        winning_point = 1
                    } else {
                        winning_point *= 2
                    }
                    println!("Found a lucky number! {}", lucky_num);
                }
            })
        });
        winning_points.push(winning_point);
        result.get_mut(index).unwrap().matches += matches;
        println!(
            "Found this many lucky numbers: {:?}",
            result.get(index).unwrap().matches
        );

        for i in 1..matches + 1 {
            println!("Adding copy to index: {}", i + 1);
            result.get_mut((index as i32 + i) as usize).unwrap().copies += 1 * copies;
        }
        println!("Done for this one\n\n")
    });

    result.iter().for_each(|i| {
        println!("{:?}", i);
    });

    let sum: i32 = result.iter().map(|s| s.copies).sum();
    println!("{}", sum);

    //println!("{:?}", winning_points.iter().sum::<i32>());
}

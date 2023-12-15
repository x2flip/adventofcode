use std::vec;

fn main() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
        .to_string();

    let mut hands = vec![];
    let rows = input.split("\n").for_each(|row| {
        let mut spl = row.split(" ");
        let handstr = spl.next().unwrap();

        let mut handstr_spl = handstr.split("");
        handstr_spl.next();
        let card1 = handstr_spl.next().unwrap().to_string();
        let card2 = handstr_spl.next().unwrap().to_string();
        let card3 = handstr_spl.next().unwrap().to_string();
        let card4 = handstr_spl.next().unwrap().to_string();
        let card5 = handstr_spl.next().unwrap().to_string();

        let bid = spl.next().unwrap().to_string().parse::<i32>().unwrap();

        let mut hand = Hand {
            card1,
            card2,
            card3,
            card4,
            card5,
            bid,
            strength: 0,
        };
        hands.push(hand)
    });

    let mut match_type = vec![];

    hands.iter_mut().for_each(|hand| {
        println!("Checking for hand: {:#?}", hand);
        let five = check_five_of_a_kind(&hand);
        if five {
            hand.strength = 20;
            match_type.push(20)
        } else {
            let four = check_four_of_a_kind(&hand);
            if four {
                hand.strength = 19;
                match_type.push(19)
            } else {
                let full_house = check_full_house(&hand);
                if full_house {
                    hand.strength = 18;
                    match_type.push(18)
                } else {
                    let three = check_three_of_a_kind(&hand);
                    if three {
                        hand.strength = 17;
                        match_type.push(17)
                    } else {
                        let two = check_two_pair(&hand);
                        if two {
                            hand.strength = 16;
                            match_type.push(16)
                        } else {
                            let pair = check_pair(&hand);
                            if pair {
                                hand.strength = 15;
                                match_type.push(15)
                            } else {
                                let high_card = get_high_card(&hand);
                                hand.strength = high_card;
                                match_type.push(high_card)
                            }
                        }
                    }
                }
            }
        }
    });

    let test_input_answer = vec![15, 17, 16, 16, 17];

    let mut current_rank = 0;

    hands.iter().for_each(|i| {
        let strength = i.strength;
        let is_match_found = false;
        hands.iter().for_each(|j| {
            if j.strength == strength {
                is_match_found = true
            }
        })

        if is_match_found {
            let a = convert_card_to_num(i.card1);
            let b = convert_card_to_num(j.card1);
            
        }
    })

    assert_eq!(match_type, test_input_answer, "The vecs do not match!");
    println!("Match Types: {:#?}", hands);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_vec_values() {}
}

#[derive(Debug)]
struct Hand {
    card1: String,
    card2: String,
    card3: String,
    card4: String,
    card5: String,
    bid: i32,
    strength: i32,
    rank: i32,
}

fn check_five_of_a_kind(hand: &Hand) -> bool {
    println!("Checking for a five of a kind");
    hand.card1 == hand.card2
        && hand.card1 == hand.card3
        && hand.card1 == hand.card4
        && hand.card1 == hand.card5
}

fn get_unique_cards(hand: &Hand) -> Vec<&String> {
    let mut unique_cards = vec![&hand.card1];

    if !unique_cards.contains(&&hand.card2) {
        unique_cards.push(&hand.card2)
    };
    if !unique_cards.contains(&&hand.card3) {
        unique_cards.push(&hand.card3)
    };
    if !unique_cards.contains(&&hand.card4) {
        unique_cards.push(&hand.card4)
    };
    if !unique_cards.contains(&&hand.card5) {
        unique_cards.push(&hand.card5)
    };

    unique_cards
}

fn check_four_of_a_kind(hand: &Hand) -> bool {
    println!("Checking for a four of a kind");
    let mut unique_cards = vec![&hand.card1];

    if unique_cards.contains(&&hand.card2) {
        unique_cards.push(&hand.card2)
    };
    if !unique_cards.contains(&&hand.card3) {
        unique_cards.push(&hand.card3)
    };
    if !unique_cards.contains(&&hand.card4) {
        unique_cards.push(&hand.card4)
    };
    if !unique_cards.contains(&&hand.card5) {
        unique_cards.push(&hand.card5)
    };

    let mut result = false;

    for i in unique_cards {
        let mut count_of_matches = 0;
        if &hand.card1 == i {
            count_of_matches += 1;
        }
        if &hand.card2 == i {
            count_of_matches += 1;
        }
        if &hand.card3 == i {
            count_of_matches += 1;
        }
        if &hand.card4 == i {
            count_of_matches += 1;
        }
        if &hand.card5 == i {
            count_of_matches += 1;
        }

        if count_of_matches == 4 {
            result = true;
        }
    }

    result
}

fn check_full_house(hand: &Hand) -> bool {
    println!("Checking for a full house");
    let unique = get_unique_cards(&hand);

    // Early return if there are more than 2 unique cards
    if unique.len() > 2 {
        return false;
    }

    let mut matches = vec![];

    let result = false;
    unique.iter().for_each(|i| {
        let mut count = 0;
        if &&hand.card1 == i {
            count += 1;
        };
        if &&hand.card2 == i {
            count += 1;
        };
        if &&hand.card3 == i {
            count += 1;
        };
        if &&hand.card4 == i {
            count += 1;
        };
        if &&hand.card5 == i {
            count += 1;
        };
        matches.push(count);
    });

    let count1 = matches.get(0).unwrap();
    let count2 = matches.get(0).unwrap();

    if (*count1 == 3 && *count2 == 2) || (*count1 == 2 && *count2 == 3) {
        return true;
    }

    result
}

fn check_three_of_a_kind(hand: &Hand) -> bool {
    println!("Checking for a three of a kind");
    let unique_cards = get_unique_cards(hand);

    let mut result = false;

    for i in unique_cards {
        let mut count_of_matches = 0;
        if &hand.card1 == i {
            count_of_matches += 1;
        }
        if &hand.card2 == i {
            count_of_matches += 1;
        }
        if &hand.card3 == i {
            count_of_matches += 1;
        }
        if &hand.card4 == i {
            count_of_matches += 1;
        }
        if &hand.card5 == i {
            count_of_matches += 1;
        }

        if count_of_matches == 3 {
            result = true;
        }
    }

    result
}

fn check_two_pair(hand: &Hand) -> bool {
    println!("Checking for a two pair");
    let mut result = false;
    let unique_cards = get_unique_cards(hand);
    println!("Two Pair Unique Cards: {:#?}", unique_cards);

    let mut pairs = vec![];

    for i in unique_cards {
        let mut count_of_matches = 0;
        if &hand.card1 == i {
            count_of_matches += 1;
        }
        if &hand.card2 == i {
            count_of_matches += 1;
        }
        if &hand.card3 == i {
            count_of_matches += 1;
        }
        if &hand.card4 == i {
            count_of_matches += 1;
        }
        if &hand.card5 == i {
            count_of_matches += 1;
        }

        if count_of_matches == 2 {
            pairs.push(1);
        };
    }

    println!("Pairs vec: For each 1, it is a pair found{:#?}", pairs);

    if pairs.len() == 2 {
        result = true;
    }

    result
}

fn check_pair(hand: &Hand) -> bool {
    println!("Checking for a pair");
    let mut result = false;
    let unique_cards = get_unique_cards(hand);

    let mut pairs = vec![];

    for i in unique_cards {
        let mut count_of_matches = 0;
        if &hand.card1 == i {
            count_of_matches += 1;
        }
        if &hand.card2 == i {
            count_of_matches += 1;
        }
        if &hand.card3 == i {
            count_of_matches += 1;
        }
        if &hand.card4 == i {
            count_of_matches += 1;
        }
        if &hand.card5 == i {
            count_of_matches += 1;
        }

        if count_of_matches == 2 {
            pairs.push(1);
        };
    }

    if pairs.len() == 1 {
        result = true;
    }

    result
}

fn convert_card_to_num(card: &str) -> i32 {
    if card == "T" {
        10
    } else if card == "J" {
        11
    } else if card == "Q" {
        12
    } else if card == "K" {
        13
    } else if card == "A" {
        14
    } else {
        return card.parse::<i32>().unwrap();
    }
}
fn convert_to_nums(hand: &Hand) -> Vec<i32> {
    let mut vec_hand = vec![];

    vec_hand.push(convert_card_to_num(&hand.card1));
    vec_hand.push(convert_card_to_num(&hand.card2));
    vec_hand.push(convert_card_to_num(&hand.card3));
    vec_hand.push(convert_card_to_num(&hand.card4));
    vec_hand.push(convert_card_to_num(&hand.card5));

    vec_hand
}
fn get_high_card(hand: &Hand) -> i32 {
    println!("Checking High Card");
    let temp_hand = convert_to_nums(&hand);
    *temp_hand.iter().max().unwrap()
}

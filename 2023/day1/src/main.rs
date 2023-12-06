use std::fs;

fn main() {
    let contents = fs::read_to_string("text2.txt".to_string()).expect("Something went wrong");

    let test_contents = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
        .to_string();

    let mut result = 0;
    let mut numbers: Vec<i32> = vec![];
    contents.split("\n").for_each(|word| {
        word.split("\n").for_each(|mut w| {
            if w == "" {
                return;
            } else {
                println!("{:?}", w);
                let num = calibrate(&w);
                println!("{:?}", num);
                numbers.push(num);
            }
        })
    });

    numbers.iter().for_each(|number| {
        result += number;
    });

    println!("{:?}", result);
}

fn calibrate(string: &str) -> i32 {
    let mut numbers = vec![];

    let parsed_string = find_and_replace(&string);

    println!("{:?}", parsed_string);

    for c in parsed_string.chars() {
        if c.is_numeric() {
            let char = c.to_string();
            numbers.push(char.parse::<i32>().unwrap())
        }
    }

    let number: i32 = numbers.first().unwrap() * 10 + numbers.last().unwrap();

    return number;
}

fn find_and_replace(word: &str) -> String {
    let mut new_word = word.to_string();
    new_word = new_word.replace("one", "o1e");
    new_word = new_word.replace("two", "t2o");
    new_word = new_word.replace("three", "th3ee");
    new_word = new_word.replace("four", "f4ur");
    new_word = new_word.replace("five", "f5ve");
    new_word = new_word.replace("six", "s6x");
    new_word = new_word.replace("seven", "se7en");
    new_word = new_word.replace("eight", "eig8t");
    new_word = new_word.replace("nine", "n9ne");

    new_word
}

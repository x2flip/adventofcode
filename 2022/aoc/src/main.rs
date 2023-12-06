use std::fs;

fn day_one() -> u32 {
    let file_name = "calories.txt".to_string();
    let contents = fs::read_to_string(file_name).expect("Error reading file");
    let mut results: Vec<u32> = contents
        .split("\n\n")
        .map(|row| row.lines().map(|x| x.parse::<u32>().unwrap()).sum::<u32>())
        .collect();
    results.sort_by(|a, b| b.cmp(a));
    let sum: u32 = results.iter().take(3).sum();
    return sum;
}
fn main() {
    let day_one_answer = day_one();
    println!("Day one answer: {}", day_one_answer);
}

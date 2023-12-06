use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let example = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
        .to_string();

    let content = fs::read_to_string("content.txt").expect("Could not read");

    let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    println!("{:?}", grid);
    let rows = grid.len();
    let cols = grid[0].len();
    let mut result = Vec::new();
    let mut seen: HashSet<(isize, isize)> = HashSet::new();
    let mut gear_ratios = Vec::new();

    for i in 0..rows {
        for j in 0..cols {
            if is_gear(grid[i][j]) {
                println!("Found gear at \nRow: {:?}\nColumn: {:?}\n", i, j);
                let mut number_count = 0;
                let mut temp_number_and_pos: HashSet<(isize, isize)> = HashSet::new();
                let mut temp_result: Vec<String> = Vec::new();
                let mut gear_ratio = 1;
                for &di in [-1, 0, 1].iter() {
                    for &dj in [-1, 0, 1].iter() {
                        if di == 0 && dj == 0 {
                            continue;
                        }
                        let row = i as isize + di;
                        let col = j as isize + dj;
                        if row >= 0
                            && col >= 0
                            && row < rows as isize
                            && col < cols as isize
                            && !seen.contains(&(row, col))
                        {
                            if let Some((number, positions)) =
                                get_number(&grid, row, col, rows, cols)
                            {
                                println!("Found number: {:?}\n Positions: {:?}", number, positions);
                                println!(
                                    "Increasing number count from: {} to: {}",
                                    number_count,
                                    number_count + 1
                                );

                                let position_clone = positions.clone();
                                temp_number_and_pos.extend(position_clone.to_owned());
                                let mut does_exist = false;

                                temp_result.iter().for_each(|row| {
                                    if row.to_owned() == number {
                                        does_exist = true;
                                    }
                                });

                                println!("Exists: {}", does_exist);

                                if !does_exist {
                                    temp_result.push(number);
                                }

                                number_count += 1;

                                println!("New Number Count: {}", number_count);
                                println!("Temp Result {:?}", temp_result);
                                println!("Temp Positions HashSet: {:?}", temp_number_and_pos);
                            }
                        }
                    }
                }
                if temp_result.len() >= 2 {
                    seen.extend(temp_number_and_pos);
                    temp_result.iter().for_each(|num| {
                        gear_ratio *= num.to_owned().parse::<i32>().unwrap();
                    });
                    temp_result
                        .iter()
                        .for_each(|item| result.push(item.clone()));
                }
                println!("Final Gear Ratio: {}\n\n", gear_ratio);
                if gear_ratio > 1 {
                    gear_ratios.push(gear_ratio);
                }
            }
        }
    }

    let mut sum = 0;
    result
        .iter()
        .for_each(|num| sum += num.to_string().parse::<i32>().unwrap());

    let sum_of_gear_ratios: i32 = gear_ratios.iter().sum();
    println!("Gear Ratio Sum: {:?}", sum_of_gear_ratios);

    println!("Collected numbers: {:?}", result);
    //println!("Sum of numbers: {:?}", sum);
}
fn is_symbol(c: char) -> bool {
    !c.is_alphanumeric() && c != '.'
}

fn is_gear(c: char) -> bool {
    c == '*'
}

fn get_number(
    grid: &[Vec<char>],
    row: isize,
    col: isize,
    rows: usize,
    cols: usize,
) -> Option<(String, Vec<(isize, isize)>)> {
    if row < 0
        || col < 0
        || row as usize >= rows
        || col as usize >= cols
        || !grid[row as usize][col as usize].is_digit(10)
    {
        return None;
    }

    let mut number = String::new();
    let mut positions = Vec::new();
    let mut c = col;

    // Extend to the left
    while c >= 1 && grid[row as usize][c as usize - 1].is_digit(10) {
        c -= 1;
    }

    // Extend to the right and collect positions
    while c < cols as isize && grid[row as usize][c as usize].is_digit(10) {
        number.push(grid[row as usize][c as usize]);
        positions.push((row, c));
        c += 1;
    }

    if !number.is_empty() {
        Some((number, positions))
    } else {
        None
    }
}

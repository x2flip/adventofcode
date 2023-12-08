use std::{
    collections::{vec_deque, HashMap},
    fs, vec,
};

fn main() {
    let test = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
        .to_string();

    let content = fs::read_to_string("input.txt").expect("Error getting input");

    let line = test.split("\n\n");

    // let mut seed_vec = vec![];

    let mut seed_ranges = vec![];

    let mut seed_starts = vec![];

    let mut seed_ranges_new = vec![];

    println!("Getting seeds");

    line.clone()
        .nth(0)
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split(" ")
        .enumerate()
        .for_each(|(index, i)| {
            if index == 0 || index % 2 == 0 {
                println!("Pushing seed into starts: {}", i);
                seed_starts.push(i.to_string().parse::<i64>().unwrap());
            } else {
                println!("Pushing seed into range: {}", i);
                seed_ranges.push(i.to_string().parse::<i64>().unwrap());
            }
            // seed_vec.push(i.to_string().parse::<i64>().unwrap())
        });

    println!("Looping through starting seeds");
    seed_starts
        .iter()
        .enumerate()
        .for_each(|(index, seed_start)| {
            println!("Adding seed starting with: {:?}", seed_start);
            let range = seed_ranges.get(index).unwrap();
            let seed_end = seed_start.to_owned() + range.to_owned() - 1;
            println!("Range: {:?}", range);
            let mut vec = vec![seed_start.to_owned(), seed_end];
            seed_ranges_new.push(vec);
        });
    println!("Seeds got: {:?}", seed_ranges_new);

    let mut seed_to_soil_range = vec![];
    let mut soil_to_fert_range = vec![];
    let mut fert_to_water_range = vec![];
    let mut water_to_light_range = vec![];
    let mut light_to_temp_range = vec![];
    let mut temp_to_hum_range = vec![];
    let mut hum_to_loc_range = vec![];

    println!("Getting Ranges");
    line.clone()
        .nth(1)
        .unwrap()
        .split(":\n")
        .nth(1)
        .unwrap()
        .split("\n")
        .for_each(|row| {
            let mut map = vec![];
            row.split(" ").for_each(|i| {
                if i != "" {
                    map.push(i.to_string().parse::<i64>().unwrap())
                }
            });
            seed_to_soil_range.push(map);
        });

    println!("Seed to Soil Ranges: {:?}", seed_to_soil_range);

    line.clone()
        .nth(2)
        .unwrap()
        .split(":\n")
        .nth(1)
        .unwrap()
        .split("\n")
        .for_each(|row| {
            let mut map = vec![];
            row.split(" ").for_each(|i| {
                if i != "" {
                    map.push(i.to_string().parse::<i64>().unwrap())
                }
            });
            soil_to_fert_range.push(map);
        });

    let mut new_soil_ranges = vec![];
    let mut n: Vec<_> = seed_ranges_new
        .iter_mut()
        .map(|range| {
            let mut ranges = vec![];

            let seed_start = range.get(0).unwrap();
            let seed_end = range.get(1).unwrap();

            seed_to_soil_range.iter().for_each(|rng2| {
                let rng2_len = rng2.get(2).unwrap();

                let source_start = rng2.get(1).unwrap();
                println!("Source Start: {:?}", source_start);
                let source_end = source_start + rng2_len;

                let dest_start = rng2.get(0).unwrap();
                println!("Dest Start: {:?}", dest_start);
                let dest_end = dest_start + rng2_len;

                let d_seed_start_source = seed_start - source_start;
                let d_seed_end_source = source_end - seed_end;

                println!("\n\nRanges to Push");
                if seed_start >= source_start {
                    println!(
                        "Seeds Start: {:?} is greater than source start: {:?}",
                        seed_start, source_start
                    );
                    if seed_end <= &source_end.to_owned() {
                        let dest_start_new = dest_start + d_seed_start_source;
                        let dest_end_new = dest_end - d_seed_end_source;

                        let new_soil_range = vec![dest_start_new, dest_end_new];

                        println!("Adding new matching soil ranges, {:?}", new_soil_range);
                        ranges.push(new_soil_range);
                    } else {
                        println!("The seed end is greater than the source end")
                    }
                } else {
                    println!("Seed range does not exist in this range!\n\n")
                }
            });
            ranges.iter().for_each(|r| {
                new_soil_ranges.push(r.clone());
            });
            return ranges;
        })
        .collect();
    println!("Soil Vecs: {:?}", n);

    let mut new_fert_ranges = vec![];

    new_soil_ranges.iter_mut().for_each(|range| {
        let mut ranges = vec![];

        let seed_start = range.get(0).unwrap();
        let seed_end = range.get(1).unwrap();

        soil_to_fert_range.iter().for_each(|rng2| {
            let rng2_len = rng2.get(2).unwrap();

            let source_start = rng2.get(1).unwrap();
            println!("Source Start: {:?}", source_start);
            let source_end = source_start + rng2_len;

            let dest_start = rng2.get(0).unwrap();
            println!("Dest Start: {:?}", dest_start);
            let dest_end = dest_start + rng2_len;

            let d_seed_start_source = seed_start - source_start;
            let d_seed_end_source = source_end - seed_end;

            println!("\n\nRanges to Push");
            if seed_start >= source_start {
                println!(
                    "Seeds Start: {:?} is greater than source start: {:?}",
                    seed_start, source_start
                );
                if seed_end <= &source_end.to_owned() {
                    let dest_start_new = dest_start + d_seed_start_source;
                    let dest_end_new = dest_end - d_seed_end_source;

                    let new_soil_range = vec![dest_start_new, dest_end_new];

                    println!("Adding new matching soil ranges, {:?}", new_soil_range);
                    ranges.push(new_soil_range);
                } else {
                    println!("The seed end is greater than the source end")
                }
            } else {
                println!("Seed range does not exist in this range!\n\n")
            }
        });
        new_fert_ranges.push(ranges);
    });
    println!("Fert Vecs: {:?}", new_fert_ranges);
    line.clone()
        .nth(3)
        .unwrap()
        .split(":\n")
        .nth(1)
        .unwrap()
        .split("\n")
        .for_each(|row| {
            let mut map = vec![];
            row.split(" ").for_each(|i| {
                if i != "" {
                    map.push(i.to_string().parse::<i64>().unwrap())
                }
            });
            fert_to_water_range.push(map);
        });
    line.clone()
        .nth(4)
        .unwrap()
        .split(":\n")
        .nth(1)
        .unwrap()
        .split("\n")
        .for_each(|row| {
            let mut map = vec![];
            row.split(" ").for_each(|i| {
                if i != "" {
                    map.push(i.to_string().parse::<i64>().unwrap())
                }
            });
            water_to_light_range.push(map);
        });
    line.clone()
        .nth(5)
        .unwrap()
        .split(":\n")
        .nth(1)
        .unwrap()
        .split("\n")
        .for_each(|row| {
            let mut map = vec![];
            row.split(" ").for_each(|i| {
                if i != "" {
                    map.push(i.to_string().parse::<i64>().unwrap())
                }
            });
            light_to_temp_range.push(map);
        });
    line.clone()
        .nth(6)
        .unwrap()
        .split(":\n")
        .nth(1)
        .unwrap()
        .split("\n")
        .for_each(|row| {
            let mut map = vec![];
            row.split(" ").for_each(|i| {
                if i != "" {
                    map.push(i.to_string().parse::<i64>().unwrap())
                }
            });
            temp_to_hum_range.push(map);
        });
    line.clone()
        .nth(7)
        .unwrap()
        .split(":\n")
        .nth(1)
        .unwrap()
        .split("\n")
        .for_each(|row| {
            let mut map = vec![];
            row.split(" ").for_each(|i| {
                if i != "" {
                    map.push(i.to_string().parse::<i64>().unwrap())
                }
            });
            hum_to_loc_range.push(map);
        });
    println!("Ranges Got!");
}

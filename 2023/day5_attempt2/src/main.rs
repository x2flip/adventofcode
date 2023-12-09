use std::{
    collections::{vec_deque, HashMap},
    fs,
    io::Split,
    vec,
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

    let mut seed_to_soil_map = vec![];
    let mut soil_to_fert_map = vec![];
    let mut fert_to_water_map = vec![];
    let mut water_to_light_map = vec![];
    let mut light_to_temp_map = vec![];
    let mut temp_to_hum_map = vec![];
    let mut hum_to_loc_map = vec![];

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
            seed_to_soil_map.push(map);
        });

    println!("Seed to Soil Ranges: {:?}", seed_to_soil_map);

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
            soil_to_fert_map.push(map);
        });

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
            fert_to_water_map.push(map);
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
            water_to_light_map.push(map);
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
            light_to_temp_map.push(map);
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
            temp_to_hum_map.push(map);
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
            hum_to_loc_map.push(map);
        });

    let mut soil_ranges: Vec<Vec<i64>> = get_mapped_ranges(seed_ranges_new, seed_to_soil_map);
    println!("\n\nSoil Ranges: {:?}", soil_ranges);
    let mut fert_ranges: Vec<Vec<i64>> = get_mapped_ranges(soil_ranges, soil_to_fert_map);
    println!("\n\nFert Ranges: {:?}", fert_ranges);
    let mut water_ranges: Vec<Vec<i64>> = get_mapped_ranges(fert_ranges, fert_to_water_map);
    println!("\n\nWater Ranges: {:?}", water_ranges);

    fn get_mapped_ranges(mut ranges: Vec<Vec<i64>>, map: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
        println!("\n\nRanges: {:?}\nMap: {:?}", ranges, map);
        ranges
            .iter_mut()
            .flat_map(|range| {
                let mut ranges = vec![];

                let mut range_start = range.get(0).unwrap();
                let range_end = range.get(1).unwrap();
                println!(
                    "\nRange Start: {:?}\nRange End: {:?}",
                    range_start, range_end
                );
                let map_source_min = map
                    .iter()
                    .map(|item| {
                        let source = item.get(1).unwrap().to_owned();
                        println!("Sources Here: {:?}", source);
                        source
                    })
                    .collect::<Vec<i64>>()
                    .into_iter()
                    .min()
                    .unwrap();

                let map_source_max = map
                    .iter()
                    .map(|item| {
                        let source = item.get(1).unwrap().to_owned();
                        let rng = item.get(2).unwrap().to_owned();
                        let src_end = source + rng;
                        src_end
                    })
                    .collect::<Vec<i64>>()
                    .into_iter()
                    .max()
                    .unwrap();

                println!(
                    "Map Source Min: {:?}\nMap Source Max: {:?}",
                    map_source_min, map_source_max
                );

                if range_start < &map_source_min {
                    // Make a new bucket for the unmapped ranges
                    let start = range_start.to_owned();
                    let end = map_source_min.to_owned() - 1;
                    let new_vec = vec![start, end];
                    println!("New Vec: {:?}", new_vec);
                    ranges.push(new_vec);

                    range_start = &map_source_min;
                };

                if range_start > &map_source_max {
                    let start = range_start.to_owned();
                    let end = range_end.to_owned();
                    let new_vec = vec![start, end];
                    println!("New Vec: {:?}", new_vec);
                    ranges.push(new_vec);
                }

                map.iter().for_each(|rng2| {
                    let rng2_len = rng2.get(2).unwrap();

                    let source_start = rng2.get(1).unwrap();
                    let source_end = source_start + rng2_len;
                    println!(
                        "Source Start: {:?}\nSource End: {:?}",
                        source_start, source_end
                    );

                    let dest_start = rng2.get(0).unwrap();
                    let dest_end = dest_start + rng2_len;
                    println!("Dest Start: {:?}\nDest End: {:?}", dest_start, dest_end);

                    let d_seed_start_source = range_start - source_start;
                    let d_seed_end_source = source_end - range_end;

                    // Need to make new buckets when source end ends inside a range and remainder
                    // of source is outside of range. Need to split and check the remainder range
                    // against all of the maps to validate nothing remains
                    println!("Ranges to Push");
                    if range_start >= source_start {
                        println!(
                            "Input Start: {:?} is greater than source start: {:?}",
                            range_start, source_start
                        );
                        if range_end <= &source_end.to_owned() {
                            let dest_start_new = dest_start + d_seed_start_source;
                            let dest_end_new = dest_end - d_seed_end_source;

                            let new_soil_range = vec![dest_start_new, dest_end_new];

                            println!("Adding new matching Dest ranges, {:?}", new_soil_range);
                            ranges.push(new_soil_range);
                        } else {
                            println!("The Input end is greater than the source end")
                        }
                    } else {
                        println!("Input range does not exist in this Source range!")
                    }
                });
                ranges.into_iter()
            })
            .collect()
    }
    println!("Ranges Got!");
}

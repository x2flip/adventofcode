use std::{collections::HashMap, fs, vec};

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

    let line = content.split("\n\n");

    line.clone().for_each(|line| println!("{:?}", line));

    let mut seed_vec = vec![];

    println!("Getting Seeds...");

    let seeds_string = line.clone().nth(0).unwrap().split(": ").last().unwrap();

    println!("Seed String: {:?}", seeds_string);

    line.clone()
        .nth(0)
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split(" ")
        .for_each(|i| seed_vec.push(i.to_string().parse::<i64>().unwrap()));

    println!("Seeds: {:?}", seed_vec);

    let mut seed_to_soil_map: Vec<Vec<i64>> = vec![];

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

    println!("Seed to Soil: {:?}", seed_to_soil_map);

    let mut seed_to_soil_hash = HashMap::new();

    seed_to_soil_map.iter().for_each(|row| {
        let dest_start = row.get(0).unwrap();
        let source_start = row.get(1).unwrap();
        let range_length = row.get(2).unwrap();

        println!("{:?}, {:?}, {:?}", dest_start, source_start, range_length);

        for i in 0..range_length.clone() {
            let hash = (source_start + i, dest_start + i);
            println!("{:?}", hash);
            seed_to_soil_hash.insert(source_start + i, dest_start + i);
        }
    });

    println!("Seed to Soil HashMap: {:?}", seed_to_soil_hash);

    let mut soil_to_fert_map: Vec<Vec<i64>> = vec![];

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

    println!("Soil to Fert {:?}", soil_to_fert_map);

    let mut soil_to_fert_hash = HashMap::new();

    soil_to_fert_map.iter().for_each(|row| {
        let dest_start = row.get(0).unwrap();
        let source_start = row.get(1).unwrap();
        let range_length = row.get(2).unwrap();

        println!("{:?}, {:?}, {:?}", dest_start, source_start, range_length);

        for i in 0..range_length.clone() {
            let hash = (source_start + i, dest_start + i);
            println!("{:?}", hash);
            soil_to_fert_hash.insert(source_start + i, dest_start + i);
        }
    });

    let mut fert_to_water: Vec<Vec<i64>> = vec![];

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
            fert_to_water.push(map);
        });

    println!("Fert to Water {:?}", fert_to_water);

    let mut fert_to_water_hash = HashMap::new();

    fert_to_water.iter().for_each(|row| {
        let dest_start = row.get(0).unwrap();
        let source_start = row.get(1).unwrap();
        let range_length = row.get(2).unwrap();

        println!("{:?}, {:?}, {:?}", dest_start, source_start, range_length);

        for i in 0..range_length.clone() {
            let hash = (source_start + i, dest_start + i);
            println!("{:?}", hash);
            fert_to_water_hash.insert(source_start + i, dest_start + i);
        }
    });

    let mut water_to_light: Vec<Vec<i64>> = vec![];

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
            water_to_light.push(map);
        });

    println!("Water to Light {:?}", water_to_light);

    let mut water_to_light_hash = HashMap::new();

    water_to_light.iter().for_each(|row| {
        let dest_start = row.get(0).unwrap();
        let source_start = row.get(1).unwrap();
        let range_length = row.get(2).unwrap();

        println!("{:?}, {:?}, {:?}", dest_start, source_start, range_length);

        for i in 0..range_length.clone() {
            let hash = (source_start + i, dest_start + i);
            println!("{:?}", hash);
            water_to_light_hash.insert(source_start + i, dest_start + i);
        }
    });

    let mut light_to_temp: Vec<Vec<i64>> = vec![];

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
            light_to_temp.push(map);
        });

    println!("Light to Temp {:?}", light_to_temp);

    let mut light_to_temp_hash = HashMap::new();

    light_to_temp.iter().for_each(|row| {
        let dest_start = row.get(0).unwrap();
        let source_start = row.get(1).unwrap();
        let range_length = row.get(2).unwrap();

        println!("{:?}, {:?}, {:?}", dest_start, source_start, range_length);

        for i in 0..range_length.clone() {
            let hash = (source_start + i, dest_start + i);
            println!("{:?}", hash);
            light_to_temp_hash.insert(source_start + i, dest_start + i);
        }
    });

    let mut temp_to_hum: Vec<Vec<i64>> = vec![];

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
            temp_to_hum.push(map);
        });

    println!("Temp to Humidity {:?}", temp_to_hum);

    let mut temp_to_hum_hash = HashMap::new();

    temp_to_hum.iter().for_each(|row| {
        let dest_start = row.get(0).unwrap();
        let source_start = row.get(1).unwrap();
        let range_length = row.get(2).unwrap();

        println!("{:?}, {:?}, {:?}", dest_start, source_start, range_length);

        for i in 0..range_length.clone() {
            let hash = (source_start + i, dest_start + i);
            println!("{:?}", hash);
            temp_to_hum_hash.insert(source_start + i, dest_start + i);
        }
    });

    let mut hum_to_loc: Vec<Vec<i64>> = vec![];

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
            hum_to_loc.push(map);
        });

    println!("Humidity to Location {:?}", hum_to_loc);

    let mut hum_to_loc_hash = HashMap::new();

    hum_to_loc.iter().for_each(|row| {
        let dest_start = row.get(0).unwrap();
        let source_start = row.get(1).unwrap();
        let range_length = row.get(2).unwrap();

        println!("{:?}, {:?}, {:?}", dest_start, source_start, range_length);

        for i in 0..range_length.clone() {
            let hash = (source_start + i, dest_start + i);
            println!("{:?}", hash);
            hum_to_loc_hash.insert(source_start + i, dest_start + i);
        }
    });

    let mut locations = vec![];

    seed_vec.iter().for_each(|seed| {
        let soil = match seed_to_soil_hash.get(seed) {
            Some(v) => v,
            None => seed,
        };

        let fert = match soil_to_fert_hash.get(soil) {
            Some(v) => v,
            None => soil,
        };

        let water = match fert_to_water_hash.get(fert) {
            Some(v) => v,
            None => fert,
        };

        let light = match water_to_light_hash.get(water) {
            Some(v) => v,
            None => water,
        };

        let temp = match light_to_temp_hash.get(light) {
            Some(v) => v,
            None => light,
        };

        let humidity = match temp_to_hum_hash.get(temp) {
            Some(v) => v,
            None => temp,
        };

        let location = match hum_to_loc_hash.get(humidity) {
            Some(v) => v,
            None => humidity,
        };

        locations.push(location);
    });
    println!("Locations {:?}", locations);
    println!("Min of locations {:?}", locations.iter().min());
}


use std::fs;
use std::collections::HashMap;

fn main() {
    let almanac_str = fs::read_to_string("input.txt").expect("Unable to read file");
    let almanac = Almanac::new(&almanac_str);
    println!("{}", almanac.get_lowest_location());
}

#[derive(Debug)]
struct Table {
    table: HashMap::<u32, u32>,
}

impl Table {
    fn new() -> Table {
        Table {table: HashMap::<u32, u32>::new()}
    }

    fn insert(mut self, key: u32, value: u32) -> Table {
        self.table.insert(key, value);
        self
    }

    fn get(&self, key: &u32) -> u32 {
        *self.table.get(key).unwrap_or(key)
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u32>,
    to_soil: Table,
    to_fertilizer: Table,
    to_water: Table,
    to_light: Table,
    to_temperature: Table,
    to_humidity: Table,
    to_location: Table,
}

impl Almanac {
    fn new(almanac_str: &str) -> Almanac {
        let seeds_and_maps_str: Vec<&str> = almanac_str.split("\n\n").collect();
        let seeds = seeds_and_maps_str[0]  // seed: s1 s2 s3 ...
                        .split(": ").collect::<Vec<&str>>()[1] // s1 s2 s3 ...
                        .split(' ').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>(); // to u32
        Almanac {
            seeds,
            to_soil: Self::get_hashmap_from_str(seeds_and_maps_str[1]),
            to_fertilizer: Self::get_hashmap_from_str(seeds_and_maps_str[2]),
            to_water: Self::get_hashmap_from_str(seeds_and_maps_str[3]),
            to_light: Self::get_hashmap_from_str(seeds_and_maps_str[4]),
            to_temperature: Self::get_hashmap_from_str(seeds_and_maps_str[5]),
            to_humidity: Self::get_hashmap_from_str(seeds_and_maps_str[6]),
            to_location:Self:: get_hashmap_from_str(seeds_and_maps_str[7]),
        }
    }

    fn get_hashmap_from_str(hashmap_str: &str) -> Table {
        let ranges: Vec<&str> = hashmap_str.split(":\n").collect::<Vec<&str>>()[1].split('\n').collect::<Vec<&str>>();
        ranges.iter().fold(Table::new(), |map_acc, range| {
            if let [destination_range_start, source_range_start, range_length] = range.split(' ').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>()[..] {
                let relative_range = 0..range_length;
                return relative_range.fold(map_acc, |map_acc2, offset| map_acc2.insert(source_range_start + offset, destination_range_start + offset));
            } else {map_acc}
        })
    }

    fn get_lowest_location(self) -> u32 {
        self.seeds.iter().map(
            |x| self.to_location.get(
                &self.to_humidity.get(
                    &self.to_temperature.get(
                        &self.to_light.get(
                            &self.to_water.get(
                                &self.to_fertilizer.get(
                                    &self.to_soil.get(&x)))))))
        ).min().unwrap()
    }
}

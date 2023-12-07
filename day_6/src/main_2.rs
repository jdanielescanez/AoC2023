
use std::fs;

fn main() {
    let races_str = fs::read_to_string("input.txt").expect("Unable to read file");
    let race = Race::new(&races_str);
    println!("{}", race.get_count());
}

struct Race {
    time: u128,
    distance: u128,
}

impl Race {
    fn new(races_str: &str) -> Race {
        if let [time, distance] = &races_str.split('\n')
                .collect::<Vec<&str>>()[0..=1]
                .iter()
                .map(|x| x.split_whitespace()
                    .collect::<Vec<&str>>()[1..]
                    .iter()
                    .fold(String::new(), |a, b| a + b)
                    .parse::<u128>().unwrap()
                ).collect::<Vec<u128>>()[..] {
            Race {
                time: *time,
                distance: *distance,
            }
        } else {
            Race {
                time: 0,
                distance: 0,
            }
        }
    }

    fn get_count(&self) -> u128 {
        let mut i = 0;
        while i < self.time - 1 && self.distance > i * (self.time - i) {
            i += 1;
        }

        let mut j = self.time;
        while j > 0 && self.distance > j * (self.time - j) {
            j -= 1;
        }
        j - i + 1
    }
}

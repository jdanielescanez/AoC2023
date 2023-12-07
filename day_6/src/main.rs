
use std::fs;

fn main() {
    let races_str = fs::read_to_string("input.txt").expect("Unable to read file");
    let races = Races::new(&races_str);
    println!("{}", races.get_product());
}

struct Races {
    races: Vec<Race>,
}

struct Race {
    time: u32,
    distance: u32,
}

impl Races {
    fn new(races_str: &str) -> Races {
        if let [times, distances] = &races_str.split('\n')
                .collect::<Vec<&str>>()[0..=1].iter()
                .map(|x| x.split_whitespace().collect::<Vec<&str>>()[1..].iter()
                        .map(|y| y.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                ).collect::<Vec<Vec<u32>>>()[..] {

            let mut races = Vec::<Race>::new();
            for (time, distance) in times.iter().zip(distances.iter()).collect::<Vec<(&u32, &u32)>>().iter() {
                races.push(Race {time: **time, distance: **distance});
            }
            Races {races}
        } else {
            Races {races: Vec::<Race>::new()}
        }
    }

    fn get_product(&self) -> u32 {
        self.races.iter().map(|x| x.get_record_cases().len() as u32).collect::<Vec<u32>>().iter().product()
    }
}

impl Race {
    fn get_posibilities(&self) -> Vec<u32> {
        let mut posibilities = Vec::<u32>::new();
        for holded_millimiters in 0..=self.time {
            posibilities.push(holded_millimiters * (self.time - holded_millimiters));
        }
        posibilities
    }

    fn get_record_cases(&self) -> Vec<u32> {
        self.get_posibilities()
            .iter()
            .filter(|x| **x > self.distance)
            .map(|x| *x)
            .collect::<Vec<u32>>()
    }
}

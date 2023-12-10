
use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
struct Journey {
    instructions: String,
    network: HashMap::<String,(String,String)>,
}

impl Journey {
    fn new(instructions: &str, network_vec_str: Vec<&str>) -> Journey {
        let mut network = HashMap::<String,(String,String)>::new();
        for node_str in network_vec_str {
            if let [origin, destiny] = node_str.split(" = (").collect::<Vec<&str>>()[0..=1] {
                if let [left, mut right] = destiny.split(", ").collect::<Vec<&str>>()[0..=1] {
                    right = &right[..right.len()-1];
                    let destiny = (left.to_string(), right.to_string());
                    network.insert(origin.to_string(), destiny);
                }

            }
        }

        Journey {instructions: instructions.to_string(), network}
    }

    fn run(&self) -> u32 {
        let mut current_node = String::from("AAA");
        let mut counter: u32 = 0;
        loop {
            for instruction in self.instructions.chars() {
                current_node = match instruction {
                    'L' => self.network[&current_node].0.clone(),
                    'R' => self.network[&current_node].1.clone(),
                    _ => panic!("{} is not a valid instruction", current_node),
                };
            }
            counter += self.instructions.len() as u32;
            if current_node == "ZZZ" {
                break;
            }
        }
        counter
    }
}

fn main() {
    let file_str = fs::read_to_string("input.txt")
        .expect("Unable to read file");
    let lines_vec: Vec<&str> = file_str.split('\n').collect::<Vec<&str>>();
    
    let instructions = lines_vec[0];
    let network_vec_str = lines_vec[2..lines_vec.len()-1].to_vec();

    let journey = Journey::new(instructions, network_vec_str);

    println!("{}", journey.run());
}

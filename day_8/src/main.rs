
use std::fs;
use std::collections::HashMap;
use num::integer;

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

    fn run(&self) -> u64 {
        let mut current_nodes = self.network.keys()
            .filter(|x| x.chars().collect::<Vec<char>>()[2] == 'A')
            .map(|x| x.clone())
            .collect::<Vec<String>>();
        let mut counter: Vec<u64> = vec![0; current_nodes.len()];
        let mut i: usize = 0;
        while i < current_nodes.len() {
            for instruction in self.instructions.chars() {
                current_nodes[i] = match instruction {
                    'L' => self.network[&current_nodes[i]].0.clone(),
                    'R' => self.network[&current_nodes[i]].1.clone(),
                    _ => panic!("{} is not a valid instruction", current_nodes[i]),
                };
                counter[i] += 1;
                if current_nodes[i].chars().collect::<Vec<char>>()[2] == 'Z' {
                    i += 1;
                }
            }
        }
        counter.iter().fold(1, |acc, number| integer::lcm(acc, *number))
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

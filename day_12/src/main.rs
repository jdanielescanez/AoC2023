
use std::fs;

#[derive(PartialEq, Debug)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

struct Record {
    data: Vec<Spring>,
    groups: Vec<u32>
}

impl Record {
    fn new(record_str: &str) -> Record {
        if let [data_str, groups_str] = record_str.split(' ').collect::<Vec<&str>>()[..] {
            Record {
                data: Self::parse_data(data_str),
                groups: Self::parse_groups(groups_str),
            }
        } else {
            Record {
                data: vec![], 
                groups: vec![],
            }
        }
    }

    fn parse_data(data_str: &str) -> Vec<Spring> {
        data_str.chars().map(|symbol|
            match symbol {
                '.' => Spring::Operational,
                '#' => Spring::Damaged,
                '?' => Spring::Unknown,
                _ => panic!("{symbol} symbol is not valid"),
            }
        ).collect::<Vec<Spring>>()
    }

    fn parse_groups(groups_str: &str) -> Vec<u32> {
        groups_str.split(',').map(|elem|
            elem.parse::<u32>().unwrap()
        ).rev().collect::<Vec<u32>>()
    }

    fn is_valid_combination(&self, i: u32) -> bool {
        let mut data_aux = String::new();
        let mut counter: i32 = -1;
        for j in (0..self.data.len()).rev() {
            data_aux.push(match self.data[j] {
                Spring::Damaged => '0',
                Spring::Operational => '1',
                Spring::Unknown => {
                    counter += 1;
                    format!("{:#032b}", i).chars().rev().collect::<Vec<char>>()[counter as usize]
                },
            });
        }

        data_aux.split('1').filter(|x| {
            x != &""
        }
        ).map(|zero_chain| 
            zero_chain.len() as u32
        ).collect::<Vec<u32>>() == self.groups
    }

    fn get_arrangements(&self) -> u32 {
        let exponent = self.data.iter().fold(0, |acc, spring|
            acc + (spring == &Spring::Unknown) as u32
        );
        (0..2_u32.pow(exponent)).fold(0, |acc, i|
            acc + self.is_valid_combination(i) as u32
        )
    }
}

fn main() {
    let file_str = fs::read_to_string("input.txt")
        .expect("Unable to read file");
    let mut record_vec_str: Vec<&str> = file_str.split('\n').collect::<Vec<&str>>();
    record_vec_str = record_vec_str[..record_vec_str.len()-1].to_vec();

    let result = record_vec_str.iter().fold(0, |acc, record_str| {
        // dbg!(&acc);
        acc + Record::new(record_str).get_arrangements()
    });
    println!("\n{result:?}");
}

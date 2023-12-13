
use std::fs;

type Position = (u32, u32);

#[derive(Clone, PartialEq, Debug)]
enum Element {
    EmptySpace,
    Galaxy,
}

#[derive(Debug)]
struct Universe {
    data: Vec<Vec<Element>>,
}

impl Universe {
    fn new(universe_vec_str: &Vec<&str>) -> Universe {
        Universe {
            data: universe_vec_str
                .iter()
                .map(|line| line
                    .chars()
                    .map(|symbol| Self::parse_symbol(symbol))
                    .collect::<Vec<Element>>()
                ).collect::<Vec<Vec<Element>>>()
        }
    }

    fn parse_symbol(symbol: char) -> Element {
        match symbol {
            '.' => Element::EmptySpace,
            '#' => Element::Galaxy,
            _ => panic!("{symbol} symbol is not valid!")
        }
    }

    fn transpose(&mut self) {
        assert!(!self.data.is_empty());
        let len = self.data[0].len();
        let mut iters: Vec<_> = self.data.clone().into_iter().map(|n| n.into_iter()).collect();
        self.data = (0..len)
            .map(|_| {
                iters
                    .iter_mut()
                    .map(|n| n.next().unwrap())
                    .collect::<Vec<Element>>()
            })
            .collect();
    }

    fn expand_by_row(&mut self) {
        let mut new_data = vec![];
        (0..self.data.len()).for_each(|i| {
            let row = self.data[i].clone();
            if row.iter().all(|element| element == &Element::EmptySpace) {
                new_data.push(row.clone());
            }
            new_data.push(row);
        });
        self.data = new_data;
    }

    fn expand_by_col(&mut self) {
        self.transpose();
        self.expand_by_row();
        self.transpose();
    }

    fn expand(&mut self) {
        self.expand_by_row();
        self.expand_by_col();
    }

    fn get_galaxy_positions(&self) -> Vec<Position> {
        let mut galaxy_positions = vec![];
        self.data.iter().enumerate().for_each(|(i, row)|
            row.iter().enumerate().for_each(|(j, element)|
                if element == &Element::Galaxy {
                    galaxy_positions.push((i as u32, j as u32))
                }
            )
        );
        galaxy_positions
    }

    fn get_distance(position: &Position, other: &Position) -> u32 {
        ((position.0 as i32 - other.0 as i32).abs() + (position.1 as i32 - other.1 as i32).abs()) as u32
    }

    fn get_shortest_paths_sum(&self) -> u32 {
        self.get_galaxy_positions().iter().enumerate().fold(0, |acc, (i, position)|
            acc + self.get_galaxy_positions()[i+1..].iter().fold(0, |acc2, position2|
                acc2 + Self::get_distance(position, position2)
            )
        )
    }
}

fn main() {
    let file_str = fs::read_to_string("input.txt")
        .expect("Unable to read file");
    let mut universe_vec_str: Vec<&str> = file_str.split('\n').collect::<Vec<&str>>();
    universe_vec_str = universe_vec_str[..universe_vec_str.len()-1].to_vec();

    let mut universe = Universe::new(&universe_vec_str);
    universe.expand();

    let result = universe.get_shortest_paths_sum();
    println!("{result:?}");
}

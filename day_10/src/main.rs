
use std::fs;

#[derive(PartialEq, Eq, Debug)]
enum Tile {
    VerticalPipe,
    HorizontalPipe,
    NorthEastBend,
    NorthWestBend,
    SouthWestBend,
    SouthEastBend,
    Ground,
    Start,
}

struct Square {
    data: Vec<Vec<Tile>>,
}

type Position = (i32, i32);

impl Square {
    fn new(square_vec_str: &Vec<&str>) -> Square {
        Square {
            data: square_vec_str
                .iter()
                .map(|line| line
                    .chars()
                    .map(|symbol| Self::parse_symbol(symbol))
                    .collect::<Vec<Tile>>()
                ).collect::<Vec<Vec<Tile>>>()
        }
    }

    fn parse_symbol(symbol: char) -> Tile {
        match symbol {
            '|' => Tile::VerticalPipe,
            '-' => Tile::HorizontalPipe,
            'L' => Tile::NorthEastBend,
            'J' => Tile::NorthWestBend,
            '7' => Tile::SouthWestBend,
            'F' => Tile::SouthEastBend,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => panic!("{} symbol is not valid!", symbol)
        }
    }

    fn get_next_position(&self, previous_position: Position, current_position: Position) -> Position {
        let (prev_i, prev_j) = previous_position;
        let (curr_i, curr_j) = current_position;
        match self.data[curr_i as usize][curr_j as usize] {
            Tile::VerticalPipe => (2 * curr_i - prev_i, curr_j),
            Tile::HorizontalPipe => (curr_i, 2 * curr_j - prev_j),
            Tile::NorthEastBend => {
                let offset = curr_i - prev_i + curr_j - prev_j;
                (prev_i + offset, prev_j + offset)
            },
            Tile::NorthWestBend => {
                (2 * curr_i - prev_i - 1, 2 * curr_j - prev_j - 1)
            },
            Tile::SouthWestBend => {
                let offset = curr_i - prev_i + curr_j - prev_j;
                (prev_i + offset, prev_j + offset)
            },
            Tile::SouthEastBend => {
                (curr_i + prev_j - curr_j, curr_j + prev_i - curr_i)
            },
            Tile::Ground => panic!("Position ({}, {}) is not in the path", curr_i, curr_j),
            Tile::Start => (curr_i, curr_j),
        }
    }

    fn get_posible_next_position(&self, i: usize, j: usize) -> Position {
        if i > 0 && vec![Tile::VerticalPipe, Tile::SouthEastBend, Tile::SouthWestBend].contains(&self.data[i - 1][j]) {
            (i as i32 - 1, j as i32)
        } else if j > 0 && vec![Tile::HorizontalPipe, Tile::NorthWestBend, Tile::SouthWestBend].contains(&self.data[i][j - 1]) {
            (i as i32, j as i32 - 1)
        } else if i < &self.data.len() - 1 && vec![Tile::VerticalPipe, Tile::NorthEastBend, Tile::NorthWestBend].contains(&self.data[i + 1][j]) {
            (i as i32 + 1, j as i32)
        } else {
            (i as i32, j as i32 + 1)
        }
    }

    fn get_steps_to_farthest_point(&self) -> usize {
        let start_i = self.data.iter().position(|row| row.iter().any(|tile| tile == &Tile::Start)).unwrap();
        let start_j = self.data[start_i].iter().position(|tile| tile == &Tile::Start).unwrap();
        let start_position = (start_i as i32, start_j as i32);

        let mut curr_position = self.get_posible_next_position(start_i, start_j);
        let mut prev_position = start_position;
        let mut counter: usize = 1;
        while curr_position != start_position {
            let next_position = self.get_next_position(prev_position, curr_position);
            prev_position = curr_position;
            curr_position = next_position;
            counter += 1;
        }
        counter / 2
    }
}

fn main() {
    let file_str = fs::read_to_string("input.txt")
        .expect("Unable to read file");
    let mut square_vec_str: Vec<&str> = file_str.split('\n').collect::<Vec<&str>>();
    square_vec_str = square_vec_str[..square_vec_str.len()-1].to_vec();
    let square = Square::new(&square_vec_str);
    let result = square.get_steps_to_farthest_point();
    println!("{}", result);
}

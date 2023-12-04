
use std::fs::File;
use std::io::{self, BufRead};
use array_tool::vec::Intersect;

fn main() {
    let mut count: i32 = 0;

    let lines = read_lines("./input.txt").unwrap();
    for line in lines.lines() {
        if let Ok(card_string) = line {
            let card = Card::new(card_string);
            count += card.get_points();
        }
    }
    println!("{}", count);
}

#[derive(Debug)]
struct Pile {
    winning: Vec::<i32>,
    owned: Vec::<i32>,
}

impl Pile {
    fn new(pile_string: &str) -> Pile {
        if let [winning, owned] = &pile_string.split(" | ").collect::<Vec<&str>>() // ["w1", "w2", ...] " | " ["o1", "o2", ...]
                             .into_iter().map(|x| x.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()) // [w1, w2, ...] [o1, o2, ...]
                             .collect::<Vec<Vec<i32>>>()[..] {

                                 return Pile { winning: winning.to_vec(), owned: owned.to_vec() };
                             } else {
                                return Pile { winning: Vec::<i32>::new(), owned: Vec::<i32>::new() }
                             }
    }

    fn get_points(self) -> i32 {
        let exponent = self.winning.intersect(self.owned).len();
        if exponent == 0 {
            0
        } else {
            i32::pow(2, (exponent - 1).try_into().unwrap())
        }
        
    }
}

#[derive(Debug)]
struct Card {
    _id: usize,
    pile: Pile,
}

impl Card {
    fn new(line: String) -> Card {
        let splitted_card_conf = line.split(": ").collect::<Vec<&str>>();
        let _id = splitted_card_conf[0] // Card x
            .split(' ').last().unwrap() // x (String)
            .parse::<usize>().unwrap(); // x (int)
        
        let pile = Pile::new(splitted_card_conf[1]);
        Card { _id, pile }
    }

    fn get_points(self) -> i32 {
        self.pile.get_points()
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines(filename: &str) -> io::Result<io::BufReader<File>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}

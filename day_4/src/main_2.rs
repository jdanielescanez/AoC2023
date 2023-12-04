
use std::fs::File;
use std::io::{self, BufRead};
use array_tool::vec::Intersect;
use std::ops::{Range};
use std::cmp::{max, min};

fn main() {
    let lines = read_lines("./input.txt").unwrap();
    let mut scratchcards_vec = Vec::<usize>::new();
    for line in lines.lines() {
        if let Ok(card_string) = line {
            scratchcards_vec.push(Card::new(card_string).get_scratchcards());
        }
    }

    let mut copy_vec: Vec<usize> = vec![1; scratchcards_vec.len()];
    for i in 0..copy_vec.len() {
        let range = intersect(i+1..i+scratchcards_vec[i]+1, 0..copy_vec.len());
        let current_value = copy_vec[i];
        copy_vec[range].iter_mut().for_each(|x| *x += current_value);
    }
    println!("{}", copy_vec.iter().sum::<usize>());
}

fn intersect(range1: Range<usize>, range2: Range<usize>) -> Range<usize> {
    max(range1.start, range2.start)..(min(range1.end, range2.end))
}

#[derive(Debug, Clone)]
struct Pile {
    winning: Vec::<i32>,
    owned: Vec::<i32>,
}

impl Pile {
    fn new(pile_string: &str) -> Pile {
        if let [winning, owned] = &pile_string.split(" | ").collect::<Vec<&str>>() // ["w1", "w2", ...] " | " ["o1", "o2", ...]
                             .into_iter().map(|x| x.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()) // [w1, w2, ...] [o1, o2, ...]
                             .collect::<Vec<Vec<i32>>>()[..] {
            return Pile {
                winning: winning.to_vec(),
                owned: owned.to_vec()
            };
        } else {
            return Pile {
                winning: Vec::<i32>::new(),
                owned: Vec::<i32>::new()
            };
        }
    }

    fn get_scratchcards(self) -> usize {
        self.winning.intersect(self.owned).len()
    }
}

#[derive(Debug, Clone)]
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

    fn get_scratchcards(self) -> usize {
        self.pile.get_scratchcards()
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines(filename: &str) -> io::Result<io::BufReader<File>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}

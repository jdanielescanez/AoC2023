
use std::fs;
use std::collections::HashMap;

#[derive(PartialEq,Eq,PartialOrd,Ord,Hash,Debug,Clone,Copy)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    hand: Vec<Card>,
}

#[derive(Debug)]
struct Game {
    players: Vec<Player>,
}

impl Game {
    fn new() -> Game {
        Game {players: Vec::<Player>::new()}
    }

    fn push(&mut self, player: Player) {
        self.players.push(player);
    }

    fn get_total_score(&self) -> u32 {
        let mut score = 0;
        for i in 0..self.players.len() {
            score += (i as u32 + 1) * self.players[self.players.len() - 1 - i].bid;
        }
        score
    }
}

impl Hand {
    fn new(hand_str: &str) -> Hand {
        let hand_vec = hand_str
            .chars()
            .map(|card_char| Card::new(&card_char))
            .collect::<Vec<Card>>();
        let hand_type = Self::get_type(&hand_vec);
        Hand {hand_type, hand: hand_vec}
    }

    fn get_type(hand_vec: &Vec<Card>) -> HandType {
        let mut hash_map = HashMap::<Card, u32>::new();
        for card in hand_vec {
            if !hash_map.contains_key(&card) {
                hash_map.insert(*card, 1);
            } else {
                hash_map.insert(*card, hash_map[&card] + 1);
            }
        }

        match hash_map.keys().len() {
            1 => HandType::FiveOfAKind,
            2 => if hash_map
                        .values()
                        .collect::<Vec<&u32>>()
                        .contains(&&4_u32) 
                            {HandType::FourOfAKind}
                 else {HandType::FullHouse},
            3 => if hash_map
                    .values()
                    .collect::<Vec<&u32>>()
                    .contains(&&3_u32)
                        {HandType::ThreeOfAKind}
                 else {HandType::TwoPair},
            4 => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Player {
    hand: Hand,
    bid: u32,
}

impl Player {
    fn new(player_str: &str) -> Player {
        if let [hand_str, bid_str] = player_str
            .split(' ')
            .collect::<Vec<&str>>()[..] {
            Player {
                hand: Self::parse_hand(hand_str),
                bid: Self::parse_bid(bid_str),
            }
        } else {
            Player {
                hand: Hand::new(""),
                bid: 0,
            }
        }
    }

    fn parse_hand(hand_str: &str) -> Hand {
        Hand::new(&hand_str)
    }

    fn parse_bid(bid_str: &str) -> u32 {
        bid_str.parse::<u32>().unwrap()
    }
}

impl Card {
    fn new(card_str: &char) -> Card {
        match card_str {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            _ => Card::Two,
        }
    }
}

fn main() {
    let file_str = fs::read_to_string("input.txt")
        .expect("Unable to read file");
    let players_str: Vec<&str> = file_str.split('\n').collect::<Vec<&str>>();
    
    let mut game = Game::new();
    for player_str in players_str[0..players_str.len()-1].iter() {
        game.push(Player::new(player_str));
    }

    game.players.sort();
    println!("{}", game.get_total_score());
}

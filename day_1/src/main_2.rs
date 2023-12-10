
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut count: i32 = 0;
    let lines = read_lines("./input.txt").unwrap();
    for line in lines.lines() {
        if let Ok(mut string) = line {
            string = change_string_by_number(string);
            count += get_number_of_line(&string);
        }
    }
    println!("{}", count);
}

fn change_string_by_number(input: String) -> String {
    input.replace("one", "one1one")
         .replace("two", "two2two")
         .replace("three", "three3three")
         .replace("four", "four4four")
         .replace("five", "five5five")
         .replace("six", "six6six")
         .replace("seven", "seven7seven")
         .replace("eight", "eight8eight")
         .replace("nine", "nine9nine")
}

fn read_lines(filename: &str) -> io::Result<io::BufReader<File>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}

fn get_number_of_line(input: &str) -> i32 {
    let mut result = String::new();
    if let Some(c) = get_first_digit(input) {
        result.push(c);
    }
    if let Some(c) = get_last_digit(input) {
        result.push(c);
    }
    result.parse::<i32>().unwrap()
}

fn get_first_digit(input: &str) -> Option<char> {
    for letter in input.chars() {
        if letter.is_numeric() {
            return Some(letter);
        }
    }
    None
}

fn get_last_digit(input: &str) -> Option<char> {
    for letter in input.chars().rev() {
        if letter.is_numeric() {
            return Some(letter);
        }
    }
    None
}

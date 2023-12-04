
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut cnt: i32 = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(string) = line {
                cnt += get_number_of_line(&string);
            }
        }
    }
    println!("{}", cnt);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_number_of_line(input: &str) -> i32 {
    let complete_string = get_first_digit(input).to_owned() + &get_last_digit(input);
    complete_string.parse::<i32>().unwrap()
}

fn get_first_digit(input: &str) -> String {
    for letter in input.chars() {
        if letter.is_numeric() {
            return letter.to_string();
        }
    }
    String::from("")
}

fn get_last_digit(input: &str) -> String {
    for letter in input.chars().rev() {
        if letter.is_numeric() {
            return letter.to_string();
        }
    }
    String::from("")
}

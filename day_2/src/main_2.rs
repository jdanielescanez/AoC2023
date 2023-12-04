
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut count: i32 = 0;

    let lines = read_lines("./input.txt").unwrap();
    for line in lines.lines() {
        if let Ok(game_string) = line {
            let game = Game::new(game_string);
            count += game.get_max_subset().get_power_sum();
        }
    }
    println!("{}", count);
}

#[derive(Debug)]
struct CubeSubset {
    red: Option<i32>,
    green: Option<i32>,
    blue: Option<i32>,
}

impl CubeSubset {
    fn new(red: Option<i32>, green: Option<i32>, blue: Option<i32>) -> CubeSubset {
        CubeSubset { red, green, blue }
    }

    fn is_beq_than(&self, other: &CubeSubset) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }

    fn get_power_sum(self) -> i32 {
        self.red.unwrap_or(0) * self.green.unwrap_or(0) * self.blue.unwrap_or(0)
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    set: Vec::<CubeSubset>,
}

impl Game {
    fn new(line: String) -> Game {
        let splitted_game_conf = line.split(": ").collect::<Vec<&str>>();
        let id = splitted_game_conf[0] // Game x
            .split(' ').last().unwrap() // x (String)
            .parse::<usize>().unwrap(); // x (int)

        let mut set = Vec::<CubeSubset>::new();
        let subsets = splitted_game_conf[1].split("; ");
        for subset in subsets {
            let mut cube_subset = CubeSubset::new(None, None, None);
            let colors = subset.split(", ");
            for color in colors {
                if let [number_cubes_str, tag] = color.split(' ').collect::<Vec<&str>>()[..] {
                    let number_cubes = Some(number_cubes_str.parse::<i32>().unwrap());
                    match tag { // cube_subset[tag] = number_cubes;
                        "red" => cube_subset.red = number_cubes,
                        "green" => cube_subset.green = number_cubes,
                        "blue" => cube_subset.blue = number_cubes,
                        _ => (),
                    }
                }
            }
            set.push(cube_subset);
        }
        Game { id, set }
    }

    fn get_max_subset(&self) -> CubeSubset {
        let (mut max_red, mut max_green, mut max_blue) = (Some(0), Some(0), Some(0));
        for subset in &self.set {
            max_red = if subset.red > max_red {subset.red} else {max_red};
            max_green = if subset.green > max_green {subset.green} else {max_green};
            max_blue = if subset.blue > max_blue {subset.blue} else {max_blue};
        }
        CubeSubset {
            red: max_red,
            green: max_green,
            blue: max_blue,
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines(filename: &str) -> io::Result<io::BufReader<File>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}

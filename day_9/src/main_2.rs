
use std::fs;

struct History {
    data: Vec<i128>,
}

impl History {
    fn new(data_str: &str) -> History {
        let data: Vec<i128> = data_str.split(' ').map(|x| x.parse::<i128>().unwrap()).rev().collect();
        
        History {
            data,
        }
    }

    fn factorial(num: i128) -> i128 {
        (1..=num).product()
    }

    fn get_next(&self) -> f64 {
        let mut function = vec![0_f64; self.data.len()];
        function[0] = self.data[0] as f64;
        for n in 1..function.len() {
            for i in 0..n {
                function[n] += - (1..=i).fold(1, |acc, j| acc * (n - j + 1) as i128) as f64 * function[i];
            }
            function[n] += self.data[n] as f64;
            function[n] /= Self::factorial(n as i128) as f64;
        }
        function.iter().enumerate().fold(0.0, |sum, (i, element)| sum as f64 + element * (1..=i).fold(1, |prod, j| prod * (function.len() - j + 1) as i128) as f64)
    }
}

fn main() {
    let file_str = fs::read_to_string("input.txt")
        .expect("Unable to read file");
    let mut lines_vec: Vec<&str> = file_str.split('\n').collect::<Vec<&str>>();
    lines_vec = lines_vec[..lines_vec.len()-1].to_vec();
    let result = lines_vec.iter().fold(0.0, |acc, line| acc + History::new(line).get_next());

    println!("{}", result);
}

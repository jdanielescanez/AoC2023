
use std::fs;

struct History {
    data: Vec<i64>,
    counter: u32,
}

impl History {
    fn new(data_str: &str) -> History {
        let data: Vec<i64> = data_str.split(' ').map(|x| x.parse::<i64>().unwrap()).collect();
        
        History {
            data,
            counter: 0,
        }
    }

    fn step(&mut self) {
        let mut data_aux = vec![0; self.data.len() - 1];
        for i in 0..data_aux.len() {
            data_aux[i] = self.data[i] - self.data[i + 1];
        }

        self.data = data_aux
    }

    fn factorial(num: i64) -> i64 {
        (1..=num).product()
    }

    fn get_function(&self) -> f64 {
        let mut function = vec![0_f64; self.data.len()];
        function[0] = self.data[0] as f64;
        for n in 1..function.len() {
            println!("It n = {}", n);
            for i in 0..n {
                println!("i = {}", i);
                function[n] += - (1..=i).fold(1, |acc, j| acc * (n - j + 1) as i64) as f64 * function[i];
            }
            function[n] += self.data[n] as f64;
            function[n] /= Self::factorial(n as i64) as f64;
            dbg!(&function);
        }
        function.iter().enumerate().fold(0.0, |sum, (i, element)| sum as f64 + element * (1..=i).fold(1, |prod, j| prod * (function.len() - j + 1) as i64) as f64)
    }
}

fn main() {
    let file_str = fs::read_to_string("example_input.txt")
        .expect("Unable to read file");
    let lines_vec: Vec<&str> = file_str.split('\n').collect::<Vec<&str>>();
    let history = History::new(lines_vec[..lines_vec.len()-1].to_vec()[1]);

    println!("{}", history.get_function());
}


use std::fs;

struct History {
    data: Vec<i128>,
    counter: u32,
}

impl History {
    fn new(data_str: &str) -> History {
        let data: Vec<i128> = data_str.split(' ').map(|x| x.parse::<i128>().unwrap()).collect();
        
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

    fn factorial(num: i128) -> i128 {
        (1..=num).product()
    }

    fn get_function(&self) -> i128 {
        let mut function = vec![0; self.data.len()];
        for n in 1..=function.len() {
            dbg!(n);
            for i in 1..n {
                function[n - 1] += (self.data[n - 1] - function[i]) / Self::factorial((n - 1) as i128);
                dbg!(&function, n - 1);
            }
        }
        function.iter().enumerate().map(|(index, element)| element * (Self::factorial((function.len() - index) as i128))).sum()
    }

}

fn main() {
    let file_str = fs::read_to_string("example_input.txt")
        .expect("Unable to read file");
    let lines_vec: Vec<&str> = file_str.split('\n').collect::<Vec<&str>>();
    let history = History::new(lines_vec[..lines_vec.len()-1].to_vec()[0]);

    println!("{}", history.get_function());
}

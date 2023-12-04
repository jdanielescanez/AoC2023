
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::{Range};
use std::cmp::{max, min};

fn main() {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let lines = read_lines("./input.txt").unwrap();
    for line in lines.lines() {
        if let Ok(string) = line {
            matrix.push(string.chars().collect());
        }
    }

    let mut matrix_symbols: Vec<Vec<Vec<i32>>> = vec![vec![Vec::<i32>::new(); matrix[0].len()]; matrix.len()];
    let mut i: usize = 0;
    while (0..matrix.len()).contains(&i) {
        let mut j: usize = 0;
        while (0..matrix[0].len()).contains(&j) {
            if matrix[i][j].is_digit(10) {
                let mut offset = 0;
                while (0..matrix[0].len()).contains(&(j + offset + 1)) &&
                        matrix[i][j + offset + 1].is_digit(10) {
                    offset += 1;
                }
                let right_limit_j = if j > 0 {j - 1} else {j};
                let range = intersect(right_limit_j..j+offset+1, 0..(matrix[0].len()-1));
                let current_number = matrix[i][j..=j+offset].iter().collect::<String>().parse::<i32>().unwrap();
                if i > 0 {
                    let range_to_vec = matrix[i - 1][range.clone()].to_vec();
                    let mut iter = range_to_vec.iter();
                    while let Some(pos) = iter.rposition(|x| *x != '.' && !x.is_digit(10)) {
                        matrix_symbols[i - 1][right_limit_j + pos].push(current_number);
                    }
                }
                if (0..matrix.len()).contains(&(i + 1)) {
                    let range_to_vec = matrix[i + 1][range.clone()].to_vec();
                    let mut iter = range_to_vec.iter();
                    while let Some(pos) = iter.rposition(|x| *x != '.' && !x.is_digit(10)) {
                        matrix_symbols[i + 1][right_limit_j + pos].push(current_number);
                    }
                }
                if j > 0 && matrix[i][j - 1] != '.' && !matrix[i][j - 1].is_digit(10) {
                    matrix_symbols[i][j - 1].push(current_number);
                }
                if (0..matrix[0].len()).contains(&(j + offset + 1)) && matrix[i][j + offset + 1] != '.' && !matrix[i][j + offset + 1].is_digit(10) {
                    matrix_symbols[i][j + offset + 1].push(current_number);
                }
                j += offset;
            }
            j += 1;
        }
        i += 1;
    }

    let mut count = 0;
    for i in 0..matrix_symbols.len() {
        for j in 0..matrix_symbols[0].len() {
            if matrix_symbols[i][j].len() == 2 {
                count += matrix_symbols[i][j][0] * matrix_symbols[i][j][1];
            }
        }
    }
    
    println!("{}", count)
}

fn intersect(range1: Range<usize>, range2: Range<usize>) -> Range<usize> {
    max(range1.start, range2.start)..(min(range1.end, range2.end) + 1)
}

fn read_lines(filename: &str) -> io::Result<io::BufReader<File>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}

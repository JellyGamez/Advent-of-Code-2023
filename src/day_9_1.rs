use std::fs::File;
use std::io::{BufRead, BufReader};

fn all_zero(numbers: &Vec<i32>) -> bool {
    for number in numbers.iter() {
        if *number != 0 {
            return false;
        }
    }
    true
}
#[allow(dead_code)]
pub fn main() {
    let mut ans = 0;
    if let Ok(file) = File::open("input.txt") {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
        for line in lines {
            let mut numbers: Vec<i32> = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
            let mut differences: Vec<i32> = Vec::new();
            let mut last = numbers.last().unwrap().clone();
            while !all_zero(&numbers) {
                for i in 0..numbers.len() - 1 {
                    differences.push(numbers[i + 1] - numbers[i]);
                }
                numbers = differences.clone();
                last += differences.last().unwrap();
                differences.clear();
            }
            ans += last;
        }
    }
    print!("{ans}");
}

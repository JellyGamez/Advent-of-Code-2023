use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn main() {
    let mut ans = 2000000000;
    let mut seeds: Vec<i64> = Vec::new();
    let mut mutated_seeds: Vec<i64> = Vec::new();
    if let Ok(file) = File::open("input.txt") {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
        for (index, line) in lines.iter().enumerate() {
            if index == 0 {
                seeds = line.split(':').nth(1).unwrap().split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();
                mutated_seeds = seeds.clone();
            }
            else if line.chars().last().unwrap_or_default().is_digit(10) {
                let numbers: Vec<i64> = line.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();
                for (i, seed) in seeds.iter().enumerate() {
                    if seed >= &numbers[1] && seed < &(&numbers[1] + &numbers[2]) {
                        mutated_seeds[i] = &numbers[0] - &numbers[1] + seed;
                    }
                }
            }
            else {
                seeds = mutated_seeds.clone();
            }
        }
    }
    for seed in mutated_seeds.iter() {
        ans = std::cmp::min(ans, *seed);
    }
    print!("{ans}");
}

use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn main() {
    let mut ans: i64 = 20000000000;
    let mut seeds: Vec<(i64, i64)> = Vec::new();
    let mut newseeds: Vec<(i64, i64)> = Vec::new();
    let mut mappings: Vec<(i64, i64, i64)> = Vec::new();
    if let Ok(file) = File::open("input.txt") {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
        for (index, line) in lines.iter().enumerate() {
            if index == 0 {
                seeds = line
                    .split(':')
                    .nth(1)
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
                    .chunks(2)
                    .map(|chunk| (chunk[0], chunk[0] + chunk[1] - 1))
                    .collect();
            }
            else if line.chars().last().unwrap_or_default().is_digit(10) {
                let numbers: Vec<i64> = line.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();
                mappings.push((numbers[0], numbers[1], numbers[2]));
            }
            else if line.len() == 0 && index != 1 {
                while seeds.len() != 0 {
                    let mut inserted = false;
                    let seed = seeds.pop().unwrap();
                    for mapping in mappings.iter() {
                        let overlap = (std::cmp::max(seed.0, mapping.1), std::cmp::min(seed.1, mapping.1 + mapping.2 - 1));
                        if overlap.0 > overlap.1 {
                            continue;
                        }
                        inserted = true;
                        newseeds.push((overlap.0 - mapping.1 + mapping.0, overlap.1 - mapping.1 + mapping.0));
                        if overlap.0 > seed.0 {
                            seeds.push((seed.0, overlap.0 - 1));
                        }
                        if overlap.1 < seed.1 {
                            seeds.push((overlap.1 + 1, seed.1));
                        }
                    }
                    if !inserted {
                        newseeds.push((seed.0, seed.1));
                    }
                }
                seeds = newseeds.clone();
                newseeds.clear();
                mappings.clear();
            }
        }
    }
    for i in seeds.iter() {
        ans = std::cmp::min(ans, i.0);
    }
    println!("{ans}");

}

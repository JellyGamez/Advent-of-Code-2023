use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[allow(dead_code)]
pub fn main() {
    let mut ans = 2000000000;
    let mut seeds: Vec<i64> = Vec::new();
    let mut map: HashMap<i64, i64> = HashMap::new();
    if let Ok(file) = File::open("input.txt") {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
        for (index, line) in lines.iter().enumerate() {
            if index == 0 {
                seeds = line.split(':').nth(1).unwrap().split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();
            }
            else if line.chars().last().unwrap_or_default().is_digit(10) {
                let numbers: Vec<i64> = line.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();
                *map.entry(numbers[1]).or_insert(0) += numbers[0] - numbers[1];
                *map.entry(numbers[1] + numbers[2]).or_insert(0) -= numbers[0] - numbers[1];
            }
        }
    }

    let mut values: Vec<(&i64, &i64)> = map.iter().collect();
    values.sort_by(|a, b| a.0.cmp(b.0));
    for item in values.iter() {
        println!("{} {}", item.0, item.1);
    }
    let mut cur = 0;
    for i in 0..100 {
        cur += *map.get(&i).unwrap_or(&0);
        println!("{} {}", i, i + cur);
    }

}

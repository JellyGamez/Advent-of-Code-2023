use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[allow(dead_code)]
pub fn main() {
    let mut ans = 0;
    if let Ok(file) = File::open("input.txt") {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
        for line in lines {
            let mut winning = HashMap::new();
            let mut matching = 0;
            let parts: Vec<&str> = line.split(':').last().unwrap().split('|').map(|s| s.trim()).collect();
            for number in parts.first().unwrap().split_whitespace().map(|s| s.trim()).collect::<Vec<&str>>() {
                winning.insert(number, true);
            }
            for number in parts.last().unwrap().split_whitespace().map(|s| s.trim()).collect::<Vec<&str>>() {
                if winning.contains_key(number) {
                    matching += 1;
                }
            }
            let mut points = 0;
            if matching != 0 {
                points = 1;
                for _ in 1..matching {
                    points *= 2;
                }
            }
            ans += points;
        }
    }
    print!("{}", ans);
}

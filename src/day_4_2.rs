use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn perpetuate(index: i32, matching: &HashMap<i32, i32>) -> i32 {
    let currently_matching = *matching.get(&index).unwrap();
    let mut ans = 1;
    for i in 1..=currently_matching {
        ans += perpetuate(index + i, &matching);
    }
    ans
}

#[allow(dead_code)]
pub fn main() {
    let mut ans = 0;
    let mut matching: HashMap<i32, i32> = HashMap::new();
    if let Ok(file) = File::open("input.txt") {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
        for (index, line) in lines.iter().enumerate() {
            let mut winning = HashMap::new();

            let mut currently_matching = 0;
            let parts: Vec<&str> = line.split(':').last().unwrap().split('|').map(|s| s.trim()).collect();
            for number in parts.first().unwrap().split_whitespace().map(|s| s.trim()).collect::<Vec<&str>>() {
                winning.insert(number, true);
            }
            for number in parts.last().unwrap().split_whitespace().map(|s| s.trim()).collect::<Vec<&str>>() {
                if winning.contains_key(number) {
                    currently_matching += 1;
                }
            }
            matching.insert(index as i32, currently_matching);
        }
        for i in 0..lines.len() {
            ans += perpetuate(i as i32, &matching);
        }

    }
    println!("{ans}");
}

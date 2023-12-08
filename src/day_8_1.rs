use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[allow(dead_code)]
pub fn main() {
    let mut left: HashMap<&str, &str> = HashMap::new();
    let mut right: HashMap<&str, &str> = HashMap::new();
    let mut route = String::new();
    if let Ok(file) = File::open("input.txt") {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
        for (index, line) in lines.iter().enumerate() {
            if index == 0 {
                route = line.clone();
            }
            else if index > 1 {
                let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
                let key = parts[0];
                let (l, r) = (&parts[1][1..=3], &parts[1][6..=8]);
                left.insert(key.clone(), l);
                right.insert(key.clone(), r);
            }
        }
        let mut node = "AAA";
        let mut step = 0;
        while node != "ZZZ" {
            if route.chars().nth(step as usize % route.len()).unwrap() == 'L' {
                if let Some(next) = left.get(&node) {
                    step += 1;
                    node = next;
                }
            }
            else {
                if let Some(next) = right.get(&node) {
                    step += 1;
                    node = next;
                }
            }
        }
        print!("{step}");
    }

}

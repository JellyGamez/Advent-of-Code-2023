use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn gcd(mut x: i128, mut y: i128) -> i128 {
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}
fn lcm(x: i128, y: i128) -> i128 {
    x * y / gcd(x, y)
}
#[allow(dead_code)]
pub fn main() {
    let mut left: HashMap<&str, &str> = HashMap::new();
    let mut right: HashMap<&str, &str> = HashMap::new();
    let mut route = String::new();
    let mut nodes: Vec<&str> = Vec::new();
    if let Ok(file) = File::open("input.txt") {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
        for (index, line) in lines.iter().enumerate() {
            if index == 0 {
                route = line.clone();
            }
            else if index > 1 {
                let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
                let key = parts[0].trim();
                if key.chars().last().unwrap() == 'A' {
                    nodes.push(key);
                }
                let (l, r) = (&parts[1][1..=3], &parts[1][6..=8]);
                left.insert(key.clone(), l);
                right.insert(key.clone(), r);
            }
        }
        let mut ans: i128 = 1;
        for node in nodes.iter() {
            let mut node = node.clone();
            let mut step = 0;
            while node.chars().last().unwrap() != 'Z' {
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
            ans = lcm(ans, step);
        }
        println!("{ans}");

    }

}

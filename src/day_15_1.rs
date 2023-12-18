use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn main() {
    let mut ans = 0;
    if let Ok(file) = File::open("input.txt") {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
        for line in lines {
            let steps = line.split(',').collect::<Vec<&str>>();
            for step in steps.iter() {
                let mut cur = 0;
                for c in step.chars() {
                    cur = (cur + c.to_ascii_lowercase() as i32) * 17 % 256;
                }
                ans += cur;
            }
        }
    }
    print!("{ans}");
}

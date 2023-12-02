use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn main() {
    let mut ans = 0;
    if let Ok(file) = File::open("input.txt") {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            let (mut r, mut g, mut b) = (0, 0, 0);
            let strings: &Vec<&str> = &line[line.find(':').unwrap() + 2..].split([',', ';']).map(|s| s.trim()).collect();
            for string in strings {
                if let Some(index) = string.find("red") {
                    r = cmp::max(r, string[..index - 1].parse::<i32>().unwrap());
                }
                if let Some(index) = string.find("green") {
                    g = cmp::max(g, string[..index - 1].parse::<i32>().unwrap());
                }
                if let Some(index) = string.find("blue") {
                    b = cmp::max(b, string[..index - 1].parse::<i32>().unwrap());
                }
            }
            ans += r * g * b;       
        }
    }
    print!("{}", ans);
}

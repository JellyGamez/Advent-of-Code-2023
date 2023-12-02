use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn main() {
    let mut ans = 0;
    const R: i32 = 12;
    const G: i32 = 13;
    const B: i32 = 14;
    if let Ok(file) = File::open("input.txt") {
        let reader = BufReader::new(file);

        for (index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            let (mut r, mut g, mut b) = (0, 0, 0);
            let strings: &Vec<&str> = &line[line.find(':').unwrap() + 2..].split(|ch| ch == ',' || ch == ';').map(|s| s.trim()).collect();
            for string in strings {
                if let Some(index) = string.find("red") {
                    r = cmp::max(r, string[0..index - 1].parse::<i32>().unwrap());
                }
                if let Some(index) = string.find("green") {
                    g = cmp::max(g, string[0..index - 1].parse::<i32>().unwrap());
                }
                if let Some(index) = string.find("blue") {
                    b = cmp::max(b, string[0..index - 1].parse::<i32>().unwrap());
                }
            }
            if r <= R && b <= B && g <= G {
                ans += index + 1;
            }         
        }
    }
    print!("{}", ans);
}

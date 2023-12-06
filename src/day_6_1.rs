use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn main() {
    let mut ans = 1;
    if let Ok(file) = File::open("input.txt") {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
        let times: Vec<i64> = lines[0].clone().split(':').nth(1).unwrap().split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();
        let distances: Vec<i64> = lines[1].clone().split(':').nth(1).unwrap().split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();
        for i in 0..times.len() {
            let time = times[i];
            let distance = distances[i];
            let mut cur = 0;
            for j in 1..=time / 2 + 1 {
                if j * (time - j) > distance {
                    cur = time + 1 - 2 * j;
                    break;
                }
            }
            println!("{cur}");
            ans *= cur;
        }
    }
    print!("{ans}");
}

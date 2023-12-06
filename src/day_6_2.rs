use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn main() {
    let mut ans = 1;
    if let Ok(file) = File::open("input.txt") {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
        let times: Vec<&str> = lines[0].split(':').nth(1).unwrap().split_whitespace().collect();
        let time: i128 = times.join("").parse::<i128>().unwrap();

        let distances: Vec<&str> = lines[1].split(':').nth(1).unwrap().split_whitespace().collect();
        let distance: i128 = distances.join("").parse::<i128>().unwrap();

        println!("{} {}", time, distance);
        for j in 1..=time / 2 + 1 {
            if j * (time - j) > distance {
                ans = time + 1 - 2 * j;
                break;
            }
        }
    }
    print!("{ans}");
}

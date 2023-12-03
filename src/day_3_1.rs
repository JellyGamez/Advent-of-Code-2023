use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn main() {
    let mut ans = 0;
    if let Ok(file) = File::open("input.txt") {

        let schematic: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
        let n = schematic.len();

        for (i, line) in schematic.iter().enumerate() {
            let mut str = String::new();
            for (j, ch) in line.chars().enumerate() {
                let range = (j - str.len()).checked_sub(1).unwrap_or(0)..j + 1;
                if ch.is_digit(10) {
                    str.push(ch);
                }
                if (!str.is_empty() && !ch.is_digit(10)) || (j == schematic[0].len() - 1 && ch.is_digit(10)) {
                    let number = str.parse::<i32>().unwrap();
                    let mut adjacent = false;
                    if i > 0 {
                        for k in range.clone() {
                            if let Some(ch) = schematic[i - 1].chars().nth(k as usize) {
                                if ch != '.' && !ch.is_digit(10) {
                                    adjacent = true;
                                }
                            }
                        }
                    }

                    if i < n - 1 {
                        for k in range.clone() {
                            if let Some(ch) = schematic[i + 1].chars().nth(k) {
                                if ch != '.' && !ch.is_digit(10) {
                                    adjacent = true;
                                }
                            }
                        }
                    }

                    for k in range.clone() {
                        if let Some(ch) = schematic[i].chars().nth(k) {
                            if ch != '.' && !ch.is_digit(10) {
                                adjacent = true;
                            }
                        }
                    }
                    if adjacent {
                        print!("{} ", number);
                        ans += number;
                    }
                    str.clear();
                }
            }
        }
    }
    print!("{}", ans);
}
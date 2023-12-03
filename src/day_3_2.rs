use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn main() {
    if let Ok(file) = File::open("input.txt") {

        let schematic: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
        let (n, m) = (schematic.len(), schematic[0].len());
        let mut data = vec![vec![(0, 1); m]; n];
        let mut ans = 0;

        for (i, line) in schematic.iter().enumerate() {
            let mut str = String::new();
            for (j, ch) in line.chars().enumerate() {
                let range = (j - str.len()).checked_sub(1).unwrap_or(0)..j + 1;
                if ch.is_digit(10) {
                    str.push(ch);
                }
                if (!str.is_empty() && !ch.is_digit(10)) || (j == m - 1 && ch.is_digit(10)) {
                    let number = str.parse::<i32>().unwrap();
                    if i > 0 {
                        for k in range.clone() {
                            if let Some(ch) = schematic[i - 1].chars().nth(k) {
                                if ch == '*' {
                                    data[i - 1][k].0 += 1;
                                    data[i - 1][k].1 *= number;
                                }
                            }
                        }
                    }

                    if i < n - 1 {
                        for k in range.clone() {
                            if let Some(ch) = schematic[i + 1].chars().nth(k) {
                                if ch == '*' {
                                    data[i + 1][k].0 += 1;
                                    data[i + 1][k].1 *= number;
                                }
                            }
                        }
                    }

                    for k in range.clone() {
                        if let Some(ch) = schematic[i].chars().nth(k) {
                            if ch == '*' {
                                data[i][k].0 += 1;
                                data[i][k].1 *= number;
                            }
                        }
                    }
                    str.clear();
                }
            }
        }
        for i in 0..n {
            for j in 0..m {
                if data[i][j].0 == 2 {
                    ans += data[i][j].1;
                }
            }
        }
        print!("{}", ans);
    }
}

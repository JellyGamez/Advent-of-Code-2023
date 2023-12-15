use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn main() {
    let mut ans = 0;
    let mut rows: [i128; 256] = [0; 256];
    let mut cols: [i128; 256] = [0; 256];
    let mut row_index: usize = 0;
    let (mut rows_cnt, mut cols_cnt) = (0, 0);

    if let Ok(file) = File::open("input.txt") {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
        for line in lines {
            if line.len() == 0 {

                for i in 0..rows_cnt - 1 {
                    if 2 * i < rows_cnt - 1 {
                        if rows[0..=i] == rows[i + 1..=2 * i + 1].iter().rev().cloned().collect::<Vec<_>>() {
                            // println!("{} row", i);
                            ans += (i + 1) * 100;
                            break;
                        }
                    }
                    else {
                        if rows[2 * (i + 1) - rows_cnt..=i] == rows[i + 1..rows_cnt].iter().rev().cloned().collect::<Vec<_>>() {
                            // println!("{} row", i);
                            ans += (i + 1) * 100;
                            break;
                        }
                    }
                }

                for i in 0..cols_cnt - 1 {
                    if 2 * i < cols_cnt - 1 {
                        if cols[0..=i] == cols[i + 1..=2 * i + 1].iter().rev().cloned().collect::<Vec<_>>() {
                            // println!("{} col", i);
                            ans += i + 1;
                            break;
                        }
                    }
                    else {
                        if cols[2 * (i + 1) - cols_cnt..=i] == cols[i + 1..cols_cnt].iter().rev().cloned().collect::<Vec<_>>() {
                            // println!("{} col", i);
                            ans += i + 1;
                            break;
                        }
                    }
                }

                for i in 0..256 {
                    rows[i] = 0;
                    cols[i] = 0;
                }
                row_index = 0;
                rows_cnt = 0;
            }
            else {
                for (col_index, c) in line.chars().enumerate() {
                    if c == '#' {
                        cols[col_index] |= 1 << row_index as i128;
                        rows[row_index] |= 1 << col_index as i128;
                    }
                }
                row_index += 1;
                rows_cnt += 1;
                cols_cnt = line.len();
            }
        }
    }
    print!("{ans}");
}

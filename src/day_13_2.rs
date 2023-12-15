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
                    let (a,  b, t);
                    let mut smudge = -1;
                    if 2 * i < rows_cnt - 1 {
                        (a, b, t) = (0, 2 * i + 1, i + 1);
                    }
                    else {
                        (a, b, t) = (2 * (i + 1) - rows_cnt, rows_cnt - 1, rows_cnt - 1 - i);
                    }
                    for j in 0..t {
                        let xor = rows[a + j] ^ rows[b - j];
                        if smudge == -1 && xor.count_ones() == 1 {
                            smudge = i as i32;
                        }
                        else if (smudge != -1 && xor.count_ones() == 1) || (xor.count_ones() > 1) {
                            smudge = -1;
                            break;
                        }
                    }
                    if smudge != -1 {
                        ans += (smudge + 1) * 100;
                        break;
                    }
                }

                for i in 0..cols_cnt - 1 {
                    let (a, b, t);
                    let mut smudge = -1;
                    if 2 * i < cols_cnt - 1 {
                        (a, b, t) = (0, 2 * i + 1, i + 1);
                    }
                    else {
                        (a, b, t) = (2 * (i + 1) - cols_cnt, cols_cnt - 1, cols_cnt - 1 - i);
                    }
                    for j in 0..t {
                        let xor = cols[a + j] ^ cols[b - j];
                        if smudge == -1 && xor.count_ones() == 1 {
                            smudge = i as i32;
                        }
                        else if (smudge != -1 && xor.count_ones() == 1) || (xor.count_ones() > 1) {
                            smudge = -1;
                            break;
                        }
                    }
                    if smudge != -1 {
                        ans += smudge + 1;
                        break;
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

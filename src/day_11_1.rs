use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn main() {
    let mut ans = 0;

    if let Ok(file) = File::open("input.txt") {
        let chart: Vec<Vec<char>> = BufReader::new(file)
            .lines()
            .map(|l| l.unwrap().chars().collect::<Vec<char>>())
            .collect();
        let mut empty_cols_prefix_sum: Vec<i32> = Vec::new();
        let mut empty_rows_prefix_sum: Vec<i32> = Vec::new();
        let mut galaxies: Vec<(usize, usize)> = Vec::new();

        let n = chart.len();
        let m = chart[0].len();

        for i in 0..n {
            let mut empty: bool = true;
            for j in 0..m {
                if chart[i][j] == '#' {
                    empty = false;
                    break;
                }
            }
            empty_rows_prefix_sum
                .push(empty_rows_prefix_sum.last().unwrap_or(&0) + i32::from(empty));
        }

        for j in 0..m {
            let mut empty: bool = true;
            for i in 0..n {
                if chart[i][j] == '#' {
                    empty = false;
                    break;
                }
            }
            empty_cols_prefix_sum
                .push(empty_cols_prefix_sum.last().unwrap_or(&0) + i32::from(empty));
        }

        for i in 0..n {
            for j in 0..m {
                if chart[i][j] == '#' {
                    galaxies.push((i, j));
                }
            }
        }

        for i in 0..galaxies.len() {
            for j in i + 1..galaxies.len() {
                let (x1, y1) = galaxies[i];
                let (x2, y2) = galaxies[j];

                ans += x1.abs_diff(x2) as i32 + y1.abs_diff(y2) as i32
                     + empty_cols_prefix_sum[cmp::max(y1, y2)]
                     - empty_cols_prefix_sum[cmp::min(y1, y2)]
                     + empty_rows_prefix_sum[cmp::max(x1, x2)]
                     - empty_rows_prefix_sum[cmp::min(x1, x2)];
            }
        }

    }
    print!("{}", ans);
}

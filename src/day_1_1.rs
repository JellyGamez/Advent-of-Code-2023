use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn main() 
{
    let mut ans = 0;
    if let Ok(file) = File::open("input.txt") {
        let reader = BufReader::new(file);

        for line in reader.lines() 
        {
            let line = line.unwrap();
            let (mut first, mut second) = (-1, -1);

            for ch in line.chars() 
            {
                if ch.is_digit(10) 
                {
                    let digit = ch.to_digit(10).unwrap() as i32;

                    if first == -1 
                    {
                        first = digit;
                    }
                    second = digit;
                }
            }
            ans += 10 * first + second;
        }
    }
    print!("{}", ans);
}

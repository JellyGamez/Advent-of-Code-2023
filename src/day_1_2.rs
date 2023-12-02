use std::fs::File;
use std::io::{BufReader,BufRead};

#[allow(dead_code)]
pub fn main() 
{
    let mut ans = 0;
    let strings = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let numbers = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    if let Ok(file) = File::open("../input.txt")
    {
        let reader = BufReader::new(file);

        for line in reader.lines() 
        {
            let line = line.unwrap();
            let (mut first, mut last, mut firstindex, mut lastindex) = (-1, -1, 10000000, -1 as isize);
            for (i, str) in strings.iter().enumerate()
            {
                if let Some(index) = line.find(str)
                {
                    if index < firstindex
                    {
                        firstindex = index;
                        first = (i + 1) as i32;
                    }
                }
                if let Some(index) = line.rfind(str)
                {
                    if index as isize > lastindex
                    {
                        lastindex = index as isize;
                        last = (i + 1) as i32;
                    }
                }
            }
            for (i, str) in numbers.iter().enumerate()
            {
                if let Some(index) = line.find(str)
                {
                    if index < firstindex
                    {
                        firstindex = index;
                        first = (i + 1) as i32;
                    }
                }
                if let Some(index) = line.rfind(str)
                {
                    if index as isize > lastindex
                    {
                        lastindex = index as isize;
                        last = (i + 1) as i32;
                    }
                }
            }
            ans += 10 * first + last;
        }
    }
    print!("{}", ans);
}
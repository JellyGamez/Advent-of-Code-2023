use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[allow(dead_code)]
pub fn main() {
    let mut ans = 0;

    if let Ok(file) = File::open("input.txt") {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
        let mut focal_lengths: HashMap<&str, u32> = HashMap::new();
        let mut boxes = vec![Vec::new(); 256];
        let steps = lines.first().unwrap().split(',').collect::<Vec<&str>>();
        for step in steps.iter() {

            if step.chars().nth(step.len() - 1).unwrap() == '-' {
                let label = &step[0..step.len() - 1];
                let mut box_index = 0;
                for c in label.chars() {
                    box_index = (box_index + c.to_ascii_lowercase() as i32) * 17 % 256;
                }
                if let Some(x) = boxes[box_index as usize].iter().position(|e| e == label) {
                    boxes[box_index as usize].remove(x);
                }
                focal_lengths.remove(label);
            }
            else {
                let label = &step[0..step.len() - 2];
                let mut box_index = 0;
                for c in label.chars() {
                    box_index = (box_index + c.to_ascii_lowercase() as i32) * 17 % 256;
                }
                focal_lengths.insert(label, step.chars().last().unwrap().to_digit(10).unwrap());
                if boxes[box_index as usize].iter().position(|e| e == label).is_none() {
                    boxes[box_index as usize].push(label.to_string());
                }
            }
        }
        for (i, _box) in boxes.iter().enumerate() {
            if _box.len() != 0 {
                println!("{}", i);

            }
            for (j, lens) in _box.iter().enumerate() {
                ans += (i as u32 + 1) * (j as u32 + 1) * focal_lengths.get(lens as &str).unwrap();
                print!("{} ", lens);
            }
            println!();
        }
    }
    print!("{ans}");
}

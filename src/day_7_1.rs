use std::fs::File;
use std::io::{BufRead, BufReader};

struct Hand {
    cards: String,
    bid: i32,
}
fn priority(s: &String) -> i32 {
    let mut ans = 0;
    let mut checked: [bool; 5] = Default::default();
    for (i, c) in s.chars().enumerate() {
        if checked[i] == false {
            let indices = s.match_indices(c).map(|(i, _)| i).collect::<Vec<usize>>();
            for index in indices.iter() {
                checked[*index] = true;
            }
            ans += indices.len() * indices.len();
        }
    }
    ans as i32
}
#[allow(dead_code)]
pub fn main() {
    let mut ans = 0;
    let mut hands: Vec<Hand> = Vec::new();
    if let Ok(file) = File::open("input.txt") {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
        for line in lines {
            let mut split = line.split_whitespace();
            let cards = split
                .next()
                .unwrap()
                .replace("A", "E")
                .replace("K", "D")
                .replace("Q", "C")
                .replace("J", "B")
                .replace("T", "A");
            let bid = split.next().unwrap().parse::<i32>().unwrap();
            hands.push(Hand { cards, bid });
        }
        hands.sort_by(|a, b| {
            let (pa, pb) = (priority(&a.cards), priority(&b.cards));
            if pa < pb {
                std::cmp::Ordering::Less
            } else if pa > pb {
                std::cmp::Ordering::Greater
            } else {
                a.cards.cmp(&b.cards)
            }
        })
    }
    for (i, hand) in hands.iter().enumerate() {
        ans += hand.bid * (i as i32 + 1);
    }
    print!("{ans}");
}

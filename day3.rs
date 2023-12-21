use std::{collections::HashMap, time::Instant};
mod data;

struct Lineparts<'a> {
    parts: Vec<(&'a str, &'a str, &'a str)>,
}

impl<'a> Lineparts<'a> {
    fn new() -> Self {
        Lineparts { parts: Vec::new() }
    }
    fn add_pair(&mut self, part1: &'a str, part2: &'a str, part3: &'a str) {
        self.parts.push((part1, part2, part3))
    }
}

fn matching_vals(s1: &str, s2: &str, s3: &str) -> Option<char> {
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();
    let chars3: Vec<char> = s3.chars().collect();

    for c1 in chars1 {
        if chars2.contains(&c1) && chars3.contains(&c1) {
            return Some(c1);
        }
    }
    None
}

fn solve() {
    let data: &str = data::DATA;

    let mut priorities: HashMap<char, i32> = HashMap::new();

    for (i, letter) in ('a'..='z').enumerate() {
        priorities.insert(letter, (i as i32) + 1);
    }

    for (i, letter) in ('A'..='Z').enumerate() {
        let i = i + 26;
        priorities.insert(letter, (i as i32) + 1);
    }

    let mut structure = Lineparts::new();

    let mut line = data.lines();

    while let (Some(line1), Some(line2), Some(line3)) = (line.next(), line.next(), line.next()) {
        let line1 = line1.trim();
        let line2 = line2.trim();
        let line3 = line3.trim();
        structure.add_pair(line1, line2, line3);
    }
    //println!("{:?}",structure.parts);

    let mut matching = Vec::new();

    for (i, j, k) in structure.parts {
        let u = matching_vals(i, j, k);
        match u {
            Some(x) => matching.push(x),
            None => println!("you suck"),
        }
    }

    let sum: i32 = matching.iter().map(|c| priorities.get(c).unwrap()).sum();

    println!("{sum}");
}

fn main() {
    let timer = Instant::now();
    solve();
    println!("{:?}", timer.elapsed());
}

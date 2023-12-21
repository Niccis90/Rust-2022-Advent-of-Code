use std::time::Instant;

mod data1;

fn checker(data: &str) -> i32 {
    let mut vector: Vec<((i32, i32), (i32, i32))> = Vec::new();
    let mut count: i32 = 0;

    for line in data.lines() {
        let line = line.trim();
        if let Some((before, after)) = line.split_once(',') {
            let range1 = parse_range(before);
            let range2 = parse_range(after);
            if is_contained(range1, range2) || is_contained(range2, range1) {
                count += 1;
            }
        }
    }

    count
}

fn parse_range(range_str: &str) -> (i32, i32) {
    let parts: Vec<i32> = range_str
        .split('-')
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    if parts.len() == 2 {
        (parts[0], parts[1])
    } else {
        (0, 0)
    }
}

fn is_contained(range1: (i32, i32), range2: (i32, i32)) -> bool {
    //range1.0 <= range2.0 && range1.1 >= range2.1 // for checking if it is entirely contained
    range1.0 <= range2.1 && range1.1 >= range2.0 // checks for overlap, that range1 does not start after range2 ends and that range1 does not end before range2 starts
}
fn solve() {
    let data: &str = data1::DATA;
    let number = checker(data);
    println!("{:?}", number);
}

fn main() {
    let timer = Instant::now();
    solve();
    println!("{:?}", timer.elapsed());
    //let data: &str = data1::DATA;
    //let number = checker(data);
    //println!("{:?}",number);
}

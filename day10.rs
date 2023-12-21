use std::usize;

use data::DATA;

mod data;

fn part1(data: &str) -> i32 {
    let mut cycle: Vec<(i32, i32)> = Vec::new();
    cycle.push((0, 1)); // index 0 = cycle count and index1 = register value
    for line in data.lines() {
        if line == "noop" {
            let mut copy = cycle.last().unwrap().clone();
            copy.0 += 1;
            cycle.push(copy);
        } else {
            let mut copy1 = cycle.last().unwrap().clone();
            copy1.0 += 2;
            let mut split = line.split_whitespace();
            let _ = split.next();
            let num: i32 = split.next().unwrap().parse().unwrap();
            copy1.1 += num;
            cycle.push(copy1);
            //println!("{:?}",num);
        }
    }
    let target_cycles = [20, 60, 100, 140, 180, 220];
    let mut output = [1; 6];
    //println!("{:?}", cycle);
    for (index, value) in target_cycles.iter().enumerate() {
        for (index2, i) in cycle.iter().enumerate() {
            if i.0 == *value {
                output[index] = i.0 * cycle[index2 - 1].1;
                //println!(" {:?} {:?}",i.0,cycle[index2-1].1);
            } else {
                let (_, element) = cycle
                    .iter()
                    .enumerate()
                    .find(|(_, x)| (x.0 as i32 - value) < 0 && (x.0 as i32 - value) > -3)
                    .unwrap();
                //println!("index {:?} element {:?}",subindex,element);
                output[index] = value * element.1;
                //println!(" {:?} {:?}",value,element.1);
            }
        }
    }
    //println!("{:?}",output);
    let sum: i32 = output.iter().sum();

    sum
}

fn part2(data: &str) -> i32 {
    let mut display = [['.'; 40]; 6];
    let mut register = 1;
    let mut cycle = 0;

    for line in data.lines() {
        if line == "noop" {
            cycle += 1;
            update_display(&mut display, cycle, register);
        } else {
            let num: i32 = line.split_whitespace().nth(1).unwrap().parse().unwrap();

            for _ in 0..2 {
                cycle += 1;
                update_display(&mut display, cycle, register);
            }
            register += num;
        }
    }

    for row in display.iter() {
        for &col in row.iter() {
            print!("{}", col);
        }
        println!();
    }

    0
}

fn update_display(display: &mut [[char; 40]; 6], cycle: i32, register: i32) {
    let row = ((cycle - 1) / 40) as usize;
    let col = ((cycle - 1) % 40) as usize;

    if ((register - 1)..=(register + 1)).contains(&(col as i32)) {
        display[row][col] = '#';
    }
}

fn main() {
    let start = std::time::Instant::now();
    let data = data::DATA;
    let test = data::TEST;
    //let part1 = part1(data);
    let part2 = part2(data);
    println!("{:?}", part2);
    println!("{:?}", start.elapsed());
}

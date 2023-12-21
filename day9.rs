use std::collections::HashSet;
use std::num;
use std::ops::Index;

mod data;
#[derive(Debug, PartialEq)]
struct head_move {
    direction: String,
    steps: u8,
}

fn translate_instructions(data: &str) -> Vec<head_move> {
    let mut temp = Vec::new();
    for line in data.lines() {
        let mut splitted = line.split_whitespace();
        let direction = splitted.next().unwrap().parse().unwrap();
        let steps: u8 = splitted.next().unwrap().parse().unwrap();
        let temp1 = head_move { direction, steps };
        temp.push(temp1)
    }

    temp
}

fn part1(instructions: &Vec<head_move>) -> u16 {
    let mut locations: HashSet<[i32; 2]> = HashSet::new();
    let mut head_start = [1; 2];
    let mut tail_start = [1; 2];
    for instruction in instructions {
        for _ in 0..instruction.steps {
            match instruction.direction.as_str() {
                "U" => head_start[1] += 1,
                "D" => head_start[1] -= 1,
                "R" => head_start[0] += 1,
                "L" => head_start[0] -= 1,
                _ => {}
            }
            let distancex: i32 = head_start[0] - tail_start[0];
            let distancey: i32 = head_start[1] - tail_start[1];

            if distancex.abs() > 1 || distancey.abs() > 1 {
                tail_start[0] += distancex.signum();
                tail_start[1] += distancey.signum();
            }
            locations.insert(tail_start);
            //println!("head: {:?} tail: {:?} xdis {:?} ydis {:?}",head_start, tail_start, distancex, distancey);
        }
    }
    locations.len() as u16
}

fn update_knot_position(head_start: [i32; 2], tail_start: [i32; 2]) -> [i32; 2] {
    let distancex = head_start[0] - tail_start[0];
    let distancey = head_start[1] - tail_start[1];

    let mut new_tail_start = tail_start;

    // Direct adjacency
    if distancex.abs() == 2 && distancey == 0 {
        new_tail_start[0] += distancex.signum();
    } else if distancey.abs() == 2 && distancex == 0 {
        new_tail_start[1] += distancey.signum();
    }
    // Diagonal movement
    else if distancex.abs() > 1 || distancey.abs() > 1 {
        new_tail_start[0] += distancex.signum();
        new_tail_start[1] += distancey.signum();
    }

    new_tail_start
}

fn part2(instructions: &Vec<head_move>) -> u16 {
    let mut locations: HashSet<[i32; 2]> = HashSet::new();
    let mut knots: [[i32; 2]; 10] = [[1; 2]; 10];

    for instruction in instructions {
        for _ in 0..instruction.steps {
            match instruction.direction.as_str() {
                "U" => knots[0][1] += 1,
                "D" => knots[0][1] -= 1,
                "R" => knots[0][0] += 1,
                "L" => knots[0][0] -= 1,
                _ => panic!("Failed to parse direction"),
            }

            for i in 1..10 {
                knots[i] = update_knot_position(knots[i - 1], knots[i]);
            }

            locations.insert(knots[9]);
        }
    }

    locations.len() as u16
}

fn main() {
    let start = std::time::Instant::now();
    let data: &str = data::DATA;
    let test: &str = data::TEST;
    let instructions = translate_instructions(data);
    //let part1 = part1(&instructions);
    let part2 = part2(&instructions);
    //println!("{:?}",instructions);
    //println!("{:?}", part1);
    println!("{:?}", part2);
    println!("{:?}", start.elapsed());
}

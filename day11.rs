use std::{collections::VecDeque, time::Instant};

mod data;

fn part1(rounds: i32) -> [i32; 8] {
    let mut apes: [VecDeque<u64>; 8] = [
        VecDeque::from([54, 89, 94]),
        VecDeque::from([66, 71]),
        VecDeque::from([76, 55, 80, 55, 55, 96, 78]),
        VecDeque::from([93, 69, 76, 66, 89, 54, 59, 94]),
        VecDeque::from([80, 54, 58, 75, 99]),
        VecDeque::from([69, 70, 85, 83]),
        VecDeque::from([89]),
        VecDeque::from([62, 80, 58, 57, 93, 56]),
    ];
    let mut counts: [i32; 8] = [0; 8];
    for _ in 0..rounds {
        if !apes[0].is_empty() {
            while !apes[0].is_empty() {
                counts[0] += 1;
                let item = apes[0][0];
                let stress_rate = item.wrapping_mul(7);
                //println!("monkey 0:{:?} {:?} ",apes[0],stress_rate);
                if stress_rate % 17 == 0 {
                    apes[0].pop_front().unwrap();
                    apes[5].push_back(stress_rate);
                } else {
                    apes[0].pop_front().unwrap();
                    apes[3].push_back(stress_rate);
                }
            }
        }
        //println!("monkey 0:{:?}",apes[0]);
        if !apes[1].is_empty() {
            while !apes[1].is_empty() {
                counts[1] += 1;
                let item = apes[1][0];
                let stress_rate = item.wrapping_add(4);
                //println!("monkey 1:{:?} {:?}",apes[1],stress_rate);
                if stress_rate % 3 == 0 {
                    apes[1].pop_front().unwrap();
                    apes[0].push_back(stress_rate);
                } else {
                    apes[1].pop_front().unwrap();
                    apes[3].push_back(stress_rate);
                }
            }
        }
        //println!("monkey 1:{:?}",apes[1]);
        if !apes[2].is_empty() {
            while !apes[2].is_empty() {
                counts[2] += 1;
                let item = apes[2][0];
                let stress_rate = item.wrapping_add(2);
                //println!("monkey 2:{:?} {:?}",apes[2],stress_rate);
                if stress_rate % 5 == 0 {
                    apes[2].pop_front().unwrap();
                    apes[7].push_back(stress_rate);
                } else {
                    apes[2].pop_front().unwrap();
                    apes[4].push_back(stress_rate);
                }
            }
        }
        //println!("monkey 2:{:?}",apes[2]);
        if !apes[3].is_empty() {
            while !apes[3].is_empty() {
                counts[3] += 1;
                let item = apes[3][0];
                let stress_rate = item.wrapping_add(7);
                //println!("monkey 3:{:?} {:?} ",apes[3],stress_rate);
                if stress_rate % 7 == 0 {
                    apes[3].pop_front().unwrap();
                    apes[5].push_back(stress_rate);
                } else {
                    apes[3].pop_front().unwrap();
                    apes[2].push_back(stress_rate);
                }
            }
        }
        if !apes[4].is_empty() {
            while !apes[4].is_empty() {
                counts[4] += 1;
                let item = apes[4][0];
                let stress_rate = item.wrapping_mul(17);
                //println!("monkey 3:{:?} {:?} ",apes[3],stress_rate);
                if stress_rate % 11 == 0 {
                    apes[4].pop_front().unwrap();
                    apes[1].push_back(stress_rate);
                } else {
                    apes[4].pop_front().unwrap();
                    apes[6].push_back(stress_rate);
                }
            }
        }
        if !apes[5].is_empty() {
            while !apes[5].is_empty() {
                counts[5] += 1;
                let item = apes[5][0];
                let stress_rate = item.wrapping_add(8);
                //println!("monkey 3:{:?} {:?} ",apes[3],stress_rate);
                if stress_rate % 19 == 0 {
                    apes[5].pop_front().unwrap();
                    apes[2].push_back(stress_rate);
                } else {
                    apes[5].pop_front().unwrap();
                    apes[7].push_back(stress_rate);
                }
            }
        }
        if !apes[6].is_empty() {
            while !apes[6].is_empty() {
                counts[6] += 1;
                let item = apes[6][0];
                let stress_rate = item.wrapping_add(6);
                //println!("monkey 3:{:?} {:?} ",apes[3],stress_rate);
                if stress_rate % 2 == 0 {
                    apes[6].pop_front().unwrap();
                    apes[0].push_back(stress_rate);
                } else {
                    apes[6].pop_front().unwrap();
                    apes[1].push_back(stress_rate);
                }
            }
        }
        if !apes[7].is_empty() {
            while !apes[7].is_empty() {
                counts[7] += 1;
                let item = apes[7][0];
                let stress_rate = item.wrapping_mul(item);
                //println!("monkey 3:{:?} {:?} ",apes[3],stress_rate);
                if stress_rate % 13 == 0 {
                    apes[7].pop_front().unwrap();
                    apes[6].push_back(stress_rate);
                } else {
                    apes[7].pop_front().unwrap();
                    apes[4].push_back(stress_rate);
                }
            }
        }
        //println!("monkey 3:{:?}",apes[3]);
        //println!("{:?}",apes);
    }
    //println!("{:?}",apes);

    counts
}

fn part2(rounds: i32) -> [i64; 8] {
    let mut apes: [VecDeque<u64>; 8] = [
        VecDeque::from([54, 89, 94]),
        VecDeque::from([66, 71]),
        VecDeque::from([76, 55, 80, 55, 55, 96, 78]),
        VecDeque::from([93, 69, 76, 66, 89, 54, 59, 94]),
        VecDeque::from([80, 54, 58, 75, 99]),
        VecDeque::from([69, 70, 85, 83]),
        VecDeque::from([89]),
        VecDeque::from([62, 80, 58, 57, 93, 56]),
    ];
    let mut counts: [i64; 8] = [0; 8];
    let lcm = 232792560;
    for _ in 0..rounds {
        if !apes[0].is_empty() {
            while !apes[0].is_empty() {
                counts[0] += 1;
                let item = apes[0][0];
                let stress_rate = item.wrapping_mul(7) % lcm;
                //println!("monkey 0:{:?} {:?} ",apes[0],stress_rate);
                if stress_rate % 17 == 0 {
                    apes[0].pop_front().unwrap();
                    apes[5].push_back(stress_rate);
                } else {
                    apes[0].pop_front().unwrap();
                    apes[3].push_back(stress_rate);
                }
            }
        }
        //println!("monkey 0:{:?}",apes[0]);
        if !apes[1].is_empty() {
            while !apes[1].is_empty() {
                counts[1] += 1;
                let item = apes[1][0];
                let stress_rate = item.wrapping_add(4) % lcm;
                //println!("monkey 1:{:?} {:?}",apes[1],stress_rate);
                if stress_rate % 3 == 0 {
                    apes[1].pop_front().unwrap();
                    apes[0].push_back(stress_rate);
                } else {
                    apes[1].pop_front().unwrap();
                    apes[3].push_back(stress_rate);
                }
            }
        }
        //println!("monkey 1:{:?}",apes[1]);
        if !apes[2].is_empty() {
            while !apes[2].is_empty() {
                counts[2] += 1;
                let item = apes[2][0];
                let stress_rate = item.wrapping_add(2) % lcm;
                //println!("monkey 2:{:?} {:?}",apes[2],stress_rate);
                if stress_rate % 5 == 0 {
                    apes[2].pop_front().unwrap();
                    apes[7].push_back(stress_rate);
                } else {
                    apes[2].pop_front().unwrap();
                    apes[4].push_back(stress_rate);
                }
            }
        }
        //println!("monkey 2:{:?}",apes[2]);
        if !apes[3].is_empty() {
            while !apes[3].is_empty() {
                counts[3] += 1;
                let item = apes[3][0];
                let stress_rate = item.wrapping_add(7) % lcm;
                //println!("monkey 3:{:?} {:?} ",apes[3],stress_rate);
                if stress_rate % 7 == 0 {
                    apes[3].pop_front().unwrap();
                    apes[5].push_back(stress_rate);
                } else {
                    apes[3].pop_front().unwrap();
                    apes[2].push_back(stress_rate);
                }
            }
        }
        if !apes[4].is_empty() {
            while !apes[4].is_empty() {
                counts[4] += 1;
                let item = apes[4][0];
                let stress_rate = item.wrapping_mul(17) % lcm;
                //println!("monkey 3:{:?} {:?} ",apes[3],stress_rate);
                if stress_rate % 11 == 0 {
                    apes[4].pop_front().unwrap();
                    apes[1].push_back(stress_rate);
                } else {
                    apes[4].pop_front().unwrap();
                    apes[6].push_back(stress_rate);
                }
            }
        }
        if !apes[5].is_empty() {
            while !apes[5].is_empty() {
                counts[5] += 1;
                let item = apes[5][0];
                let stress_rate = item.wrapping_add(8) % lcm;
                //println!("monkey 3:{:?} {:?} ",apes[3],stress_rate);
                if stress_rate % 19 == 0 {
                    apes[5].pop_front().unwrap();
                    apes[2].push_back(stress_rate);
                } else {
                    apes[5].pop_front().unwrap();
                    apes[7].push_back(stress_rate);
                }
            }
        }
        if !apes[6].is_empty() {
            while !apes[6].is_empty() {
                counts[6] += 1;
                let item = apes[6][0];
                let stress_rate = item.wrapping_add(6) % lcm;
                //println!("monkey 3:{:?} {:?} ",apes[3],stress_rate);
                if stress_rate % 2 == 0 {
                    apes[6].pop_front().unwrap();
                    apes[0].push_back(stress_rate);
                } else {
                    apes[6].pop_front().unwrap();
                    apes[1].push_back(stress_rate);
                }
            }
        }
        if !apes[7].is_empty() {
            while !apes[7].is_empty() {
                counts[7] += 1;
                let item = apes[7][0];
                let stress_rate = item.wrapping_mul(item) % lcm;
                //println!("monkey 3:{:?} {:?} ",apes[3],stress_rate);
                if stress_rate % 13 == 0 {
                    apes[7].pop_front().unwrap();
                    apes[6].push_back(stress_rate);
                } else {
                    apes[7].pop_front().unwrap();
                    apes[4].push_back(stress_rate);
                }
            }
        }
        //println!("monkey 3:{:?}",apes[3]);
        //println!("{:?}",apes);
    }
    //println!("{:?}",apes);
    counts
}

fn product(mut data: [i64; 8]) -> i64 {
    data.sort_unstable();
    data[6] * data[7]
}

fn main() {
    let now = Instant::now();
    //let part1 = part1(20);
    let part2 = part2(10000);
    //println!("{:?}", part1);
    println!("{:?}", part2);
    println!("{:?}", product(part2));
    println!("{:?}", now.elapsed());
}

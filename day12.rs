use std::{collections::HashMap, time::Instant};
mod data;
#[allow(unused_variables)]
#[allow(non_camel_case_types)]

enum Elevation {
    S,
    a,
    b,
    c,
    d,
    e,
    f,
    g,
    h,
    i,
    j,
    k,
    l,
    m,
    n,
    o,
    p,
    q,
    r,
    s,
    t,
    u,
    v,
    w,
    x,
    y,
    z,
    E,
}

impl Elevation {
    fn from_char(value: char) -> Option<Elevation> {
        match value {
            'S' => Some(Elevation::S),
            'a' => Some(Elevation::a),
            'b' => Some(Elevation::b),
            'c' => Some(Elevation::c),
            'd' => Some(Elevation::d),
            'e' => Some(Elevation::e),
            'f' => Some(Elevation::f),
            'g' => Some(Elevation::g),
            'h' => Some(Elevation::h),
            'i' => Some(Elevation::i),
            'j' => Some(Elevation::j),
            'k' => Some(Elevation::k),
            'l' => Some(Elevation::l),
            'm' => Some(Elevation::m),
            'n' => Some(Elevation::n),
            'o' => Some(Elevation::o),
            'p' => Some(Elevation::p),
            'q' => Some(Elevation::q),
            'r' => Some(Elevation::r),
            's' => Some(Elevation::s),
            't' => Some(Elevation::t),
            'u' => Some(Elevation::u),
            'v' => Some(Elevation::v),
            'w' => Some(Elevation::w),
            'x' => Some(Elevation::x),
            'y' => Some(Elevation::y),
            'z' => Some(Elevation::z),
            'E' => Some(Elevation::E),

            _ => None,
        }
    }
    fn get_value(&self) -> i8 {
        match self {
            Elevation::S => 1, // change to zero on part1
            Elevation::a => 1,
            Elevation::b => 2,
            Elevation::c => 3,
            Elevation::d => 4,
            Elevation::e => 5,
            Elevation::f => 6,
            Elevation::g => 7,
            Elevation::h => 8,
            Elevation::i => 9,
            Elevation::j => 10,
            Elevation::k => 11,
            Elevation::l => 12,
            Elevation::m => 13,
            Elevation::n => 14,
            Elevation::o => 15,
            Elevation::p => 16,
            Elevation::q => 17,
            Elevation::r => 18,
            Elevation::s => 19,
            Elevation::t => 20,
            Elevation::u => 21,
            Elevation::v => 22,
            Elevation::w => 23,
            Elevation::x => 24,
            Elevation::y => 25,
            Elevation::z => 26,
            Elevation::E => 27,
        }
    }
}

fn parse(data: &str) -> [[u8; 171]; 41] {
    let mut mountain: [[u8; 171]; 41] = [[0; 171]; 41];

    for (i, line) in data.lines().enumerate() {
        let mut iter = line
            .chars()
            .filter_map(Elevation::from_char)
            .map(|val| val.get_value());
        for j in 0..mountain[i].len() {
            if let Some(value) = iter.next() {
                mountain[i][j] = value as u8;
            } else {
                break;
            }
        }
    }

    mountain
}

fn get_surrounding_points(pos: (usize, usize)) -> Vec<(usize, usize)> {
    let dir = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let istart = (pos.0 as i32, pos.1 as i32);
    let temp = dir
        .iter()
        .map(|d| (istart.0 + d.0, istart.1 + d.1))
        .filter(|pos| pos.0 >= 0 && pos.1 >= 0 && pos.0 < 41 && pos.1 < 171)
        .map(|pos| (pos.0 as usize, pos.1 as usize))
        .collect();
    temp
}

fn find_position(mountain: &[[u8; 171]; 41], target_elevation: u8) -> Vec<(usize, usize)> {
    let mut temp = Vec::new();
    for (i, row) in mountain.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == target_elevation {
                temp.push((i, j));
            }
        }
    }
    temp
}

fn part1_2(data: &str) -> usize {
    let mountain = parse(data);
    let start = find_position(&mountain, 1);
    let end = find_position(&mountain, 27);
    let mut to_visit = Vec::new();
    let mut shortest: HashMap<(usize, usize), usize> = HashMap::new();

    for i in start {
        to_visit.extend(get_surrounding_points(i));
        shortest.insert(i, 0);
    }

    while let Some(loc) = to_visit.pop() {
        let curr_elevation = mountain[loc.0][loc.1];
        let points = get_surrounding_points(loc);
        let valid = points
            .iter()
            .filter(|pos| mountain[pos.0][pos.1] + 1 >= curr_elevation)
            .map(|pos| *pos)
            .collect::<Vec<(usize, usize)>>();
        let new_path_dist = valid.iter().filter_map(|pos| shortest.get(pos)).min();

        if new_path_dist.is_none() {
            continue;
        }

        let new_path_dist = new_path_dist.unwrap() + 1;

        let curr_path_dist = shortest.entry(loc).or_insert(usize::MAX);

        if *curr_path_dist > new_path_dist {
            *curr_path_dist = new_path_dist;
            to_visit.extend(points.iter());
        }
    }

    let answer = shortest.get(&end[0]).unwrap();

    *answer
}

fn main() {
    let time = Instant::now();
    let data: &str = data::DATA;
    let test = data::test;
    let part1_2 = part1_2(data);
    println!("{:?}", part1_2);

    println!("{:?}", time.elapsed());
}

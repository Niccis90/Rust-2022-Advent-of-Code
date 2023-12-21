mod data;

fn translate_matrix(data: &str) -> ([[u8; 99]; 99], [[u8; 99]; 99]) {
    let lines = data.lines();
    let mut temp1 = [[0; 99]; 99];
    let mut temp2 = [[0; 99]; 99];
    for (i, line) in lines.enumerate() {
        for (j, char) in line.chars().enumerate() {
            let num = char.to_digit(10).unwrap() as u8;
            temp1[i][j] = num;
            temp2[j][i] = num;
        }
    }
    return (temp1, temp2);
}

fn part1(matrix1: &[[u8; 99]; 99], matrix2: &[[u8; 99]; 99]) -> u16 {
    let mut counter = 392;
    for i in 1..98 {
        for j in 1..98 {
            let (left, righttemp) = matrix1[i].split_at(j);
            let right = &righttemp[1..];
            let (up, downtemp) = matrix2[j].split_at(i);
            let down = &downtemp[1..];
            let target = matrix1[i][j];
            if left.iter().all(|x| x < &target)
                || right.iter().all(|x| x < &target)
                || up.iter().all(|x| x < &target)
                || down.iter().all(|x| x < &target)
            {
                counter += 1;
            }
        }
    }
    counter
}

fn part2(matrix1: &[[u8; 99]; 99], matrix2: &[[u8; 99]; 99]) -> u32 {
    let mut scenic_score = 0;
    for i in 1..98 {
        for j in 1..98 {
            let mut leftview: u32 = 0;
            let mut rightview: u32 = 0;
            let mut upview: u32 = 0;
            let mut downview: u32 = 0;
            let (left, righttemp) = matrix1[i].split_at(j);
            let right = &righttemp[1..];
            let (up, downtemp) = matrix2[j].split_at(i);
            let down = &downtemp[1..];
            let target = matrix1[i][j];

            for (index, num) in left.iter().rev().enumerate() {
                if num < &target && index != left.len() - 1 {
                    leftview += 1;
                } else if num < &target && index == left.len() - 1 {
                    leftview += 1;
                    break;
                } else {
                    leftview += 1;
                    break;
                }
            }
            for (index, num) in right.iter().enumerate() {
                if num < &target && index != right.len() - 1 {
                    rightview += 1;
                } else if num < &target && index == right.len() - 1 {
                    rightview += 1;
                    break;
                } else {
                    rightview += 1;
                    break;
                }
            }
            for (index, num) in up.iter().rev().enumerate() {
                if num < &target && index != up.len() - 1 {
                    upview += 1;
                } else if num < &target && index == up.len() - 1 {
                    upview += 1;
                    break;
                } else {
                    upview += 1;
                    break;
                }
            }
            for (index, num) in down.iter().enumerate() {
                if num < &target && index != down.len() - 1 {
                    downview += 1;
                } else if num < &target && index == down.len() - 1 {
                    downview += 1;
                    break;
                } else {
                    downview += 1;
                    break;
                }
            }
            let score: u32 = leftview * rightview * upview * downview;
            if score > scenic_score {
                scenic_score = score;
            }
        }
    }

    scenic_score
}

fn main() {
    let start = std::time::Instant::now();
    let data: &str = data::DATA;
    let (matrix1, matrix2) = translate_matrix(data);
    let part1 = part1(&matrix1, &matrix2);
    let part2 = part2(&matrix1, &matrix2);
    println!("{:?}", part1);
    println!("{:?}", part2);
    println!("{:?}", start.elapsed());
    //assert_eq!(part1,1695);
}

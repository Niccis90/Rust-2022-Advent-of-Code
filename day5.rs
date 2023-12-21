mod data;

#[derive(Debug)]
pub struct Instruction {
    _move: u8,
    from: u8,
    to: u8,
}

fn instruction_translator(data: &str) -> Vec<Instruction> {
    let mut temp = Vec::new();
    for line in data.lines() {
        let _move: u8 = line.split_whitespace().nth(1).unwrap().parse().unwrap();
        let from: u8 = line.split_whitespace().nth(3).unwrap().parse().unwrap();
        let to: u8 = line.split_whitespace().nth(5).unwrap().parse().unwrap();
        let instruction = Instruction { _move, from, to };
        temp.push(instruction);
    }
    return temp;
}

fn translate_matrix(data: &str) -> Vec<Vec<char>> {
    let mut temp1: Vec<Vec<char>> = Vec::new();
    for i in [1, 5, 9, 13, 17, 21, 25, 29, 33] {
        let mut temp2: Vec<char> = Vec::new();
        for line in data.lines().rev().skip(1) {
            if let Some(character) = line.chars().nth(i) {
                if !character.is_whitespace() {
                    temp2.push(character);
                }
            }
        }
        temp1.push(temp2);
    }

    temp1
}

fn crane(crates: Vec<Vec<char>>, instructions: Vec<Instruction>) -> Vec<Vec<char>> {
    let mut newcrates = crates;
    for instruction in &instructions {
        let quantity = instruction._move;
        let from = instruction.from as usize;
        let to = instruction.to as usize;
        let mut temp: Vec<char> = Vec::new();
        for _ in 0..quantity {
            let x: char = newcrates[(from - 1) as usize].pop().unwrap();
            //newcrates[(to - 1) as usize].push(x);
            temp.push(x)
        }
        temp.reverse();
        newcrates[(to - 1) as usize].append(&mut temp);
    }
    newcrates
}

fn shower(newcrate: Vec<Vec<char>>) -> Vec<char> {
    let mut letter: Vec<char> = Vec::new();
    for vec in &newcrate {
        if let Some(&last) = vec.last() {
            letter.push(last);
        }
    }
    letter
}

fn main() {
    let data: &str = data::DATA;
    let crates: &str = data::CRATES;
    let u = instruction_translator(data);
    //println!("{:?}", u);
    let b = translate_matrix(crates);
    //println!("{:?}", b)
    let answer = crane(b, u);
    //println!("{:?}", answer);
    let show = shower(answer);
    println!("{:?}", show);
}

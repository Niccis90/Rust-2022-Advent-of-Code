use std::collections::HashSet;

mod data;

fn packet_maker(data: &str) -> Vec<usize> {
    let mut index = Vec::new();
    let data_len = data.chars().count();
    for i in 0..data_len - 3 {
        let l1 = data.chars().nth(i).unwrap();
        let l2 = data.chars().nth(i + 1).unwrap();
        let l3 = data.chars().nth(i + 2).unwrap();
        let l4 = data.chars().nth(i + 3).unwrap();
        if l1 != l2 && l1 != l3 && l1 != l4 && l2 != l3 && l2 != l4 && l3 != l4 {
            index.push(i + 4);
            break;
        }
    }
    index
}

fn message_maker(data: &str) -> Vec<usize> {
    let mut index = Vec::new();
    let data_len = data.chars().count();
    for i in 0..data_len {
        let group: Vec<char> = data.chars().skip(i).take(14).collect();
        let group_set: HashSet<_> = group.iter().collect();
        if group.len() == group_set.len() {
            index.push(i + 14);
            break;
        }
    }
    index
}

fn main() {
    let data: &str = data::DATA;
    let data1: &str = data::DATA1;
    let answer = message_maker(data);
    println!("{:?}", answer);
}

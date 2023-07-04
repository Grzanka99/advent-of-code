use std::{collections::HashSet, fs};

fn to_priority(ch: char) -> Option<u8> {
    match ch {
        'a'..='z' => Some(ch as u8 - 90 - 6),
        'A'..='Z' => Some(ch as u8 - 26 - 12),
        _ => None,
    }
}

fn main() {
    let file = fs::read_to_string("vals").expect("Should be able to read file");
    let lines = file.lines();

    let mut sum: usize = 0;

    for line in lines {
        let (left, right) = line.split_at(line.len() / 2);

        let common_letters: HashSet<u8> = left
            .chars()
            .clone()
            .filter_map(|c| match right.contains(c) {
                true => to_priority(c),
                false => None,
            })
            .collect();

        for entry in common_letters.into_iter() {
            sum += entry as usize;
        }
    }

    println!("sum: {:?}", sum);
}

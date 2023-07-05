use std::{collections::HashSet, fs};

fn to_priority(ch: char) -> Option<u8> {
    match ch {
        'a'..='z' => Some(ch as u8 - 90 - 6),
        'A'..='Z' => Some(ch as u8 - 26 - 12),
        _ => None,
    }
}

fn common_letters(a: &str, b: &str, c: &str) -> char {
    let common_ab: HashSet<char> = a
        .chars()
        .filter_map(|ch| match b.contains(ch) {
            true => Some(ch),
            false => None,
        })
        .collect();

    return c
        .chars()
        .find_map(|ch| match common_ab.contains(&ch) {
            true => Some(ch),
            false => None,
        })
        .unwrap();
}

fn main() {
    let file = fs::read_to_string("vals").expect("Should be able to read file");
    let lines = file.lines();

    let mut grouped_lines: Vec<Vec<&str>> = vec![];
    let mut buff: Vec<&str> = vec![];
    for (it, line) in lines.enumerate() {
        buff.push(line);

        if (it + 1) % 3 == 0 {
            grouped_lines.push(buff);
            buff = vec![];
        }
    }

    let mut sum: usize = 0;
    for group in grouped_lines {
        let common = common_letters(group[0], group[1], group[2]);
        let priority = to_priority(common).unwrap();
        sum += priority as usize;
    }

    println!("sum: {:?}", sum);
}

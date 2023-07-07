use std::fs;

trait GetCharAt {
    fn chat_at(self: &Self, index: usize) -> char;
}

impl GetCharAt for str {
    fn chat_at(self: &Self, index: usize) -> char {
        return self.as_bytes()[index] as char;
    }
}

#[derive(Debug)]
struct Set {
    values: Vec<char>,
}

impl Set {
    fn insert(&mut self, v: char) {
        if !self.values.contains(&v) {
            self.values.push(v);
        }
    }

    fn len(self) -> usize {
        return self.values.len();
    }

    pub fn new() -> Set {
        return Set { values: vec![] };
    }
}

fn main() {
    let file = fs::read_to_string("vals").expect("Should read the file");

    let mut res = 0;

    for line in file.lines() {
        for i in 0..line.len() - 14 {
            let mut unique_chars = Set::new();

            for offset in 0..14 {
                unique_chars.insert(line.chat_at(i + offset));
            }

            if unique_chars.len() == 14 {
                res = i + 14;
                break;
            }
        }
    }

    println!("{}", res);
}

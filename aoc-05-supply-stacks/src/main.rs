use std::fs;
use std::str;

fn first_meaningfull_char(line: &str) -> Option<char> {
    let mut res = ' ';
    for symbol in line.chars() {
        if symbol != ' ' {
            res = symbol;
            break;
        }
    }

    return match res {
        ' ' => None,
        _ => Some(res),
    };
}

// fn extract_supplies()

fn main() {
    let file = fs::read_to_string("vals").expect("Should read file");
    let lines = file.lines();

    let size = 9;
    let mut supplies: Vec<Vec<&str>> = Vec::with_capacity(size);
    let mut tokens: Vec<Vec<usize>> = vec![];

    for _ in 0..size {
        supplies.push(vec![]);
    }

    for line in lines {
        let frist_char = match first_meaningfull_char(line) {
            Some(x) => x,
            None => ' ',
        };

        match frist_char {
            '[' => {
                let to_chunks = line
                    .as_bytes()
                    .chunks(4)
                    .map(str::from_utf8)
                    .collect::<Result<Vec<&str>, _>>()
                    .unwrap();

                for (index, chunk) in to_chunks.iter().enumerate() {
                    if !chunk.trim().is_empty() {
                        supplies[index].push(chunk.trim());
                    }
                    // print!("i: {}, c: {}", index, chunk);
                }
            }
            'm' => {
                let to_tokens: Vec<&str> = line.split_whitespace().collect();
                let to_chunks = to_tokens
                    .chunks(2)
                    .map(|x| (x[0], x[1]))
                    .collect::<Vec<(&str, &str)>>();

                let mut part: Vec<usize> = vec![];
                for (_, val) in to_chunks {
                    let to_value = String::from(val).parse::<usize>().unwrap();
                    part.push(to_value);
                }

                tokens.push(part);
            }
            _ => (),
        }
    }

    for context in tokens {
        let mut to_merge: Vec<&str> = vec![];
        // println!("{:?}", context);

        for _ in 0..context[0] {
            supplies[context[1] - 1].reverse();
            to_merge.push(&supplies[context[1] - 1].pop().unwrap());
            supplies[context[1] - 1].reverse();
        }
        // print!("{:?}", to_merge);

        supplies[context[2] - 1].reverse();

        to_merge.reverse();
        for item in to_merge {
            supplies[context[2] - 1].push(item);
        }

        supplies[context[2] - 1].reverse();
        // break;
    }

    let mut res: String = "".to_owned();

    for mag in &supplies {
        let supply = mag[0].as_bytes()[1] as char;
        res.push(supply);
    }

    println!("{}", res);
}

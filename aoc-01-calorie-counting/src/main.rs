use std::fs;

fn main() {
    let file_path = "src/elves.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read file");
    let lines = contents.lines();

    let mut acc = 0;
    let mut max = [0, 0, 0];
    for line in lines {
        match line {
            "" => match acc {
                n if n > max[0] => {
                    max[0] = n;
                    acc = 0
                }
                n if n > max[1] => {
                    max[1] = n;
                    acc = 0
                }
                n if n > max[2] => {
                    max[2] = n;
                    acc = 0
                }
                _ => acc = 0,
            },
            n => acc += n.to_string().parse::<i32>().unwrap(),
        }
    }

    println!("Top 1: {}", max[0]);
    println!("Top 3: {}", max[0] + max[1] + max[2]);
}

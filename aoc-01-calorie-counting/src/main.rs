use std::collections::BinaryHeap;
use std::fs;

fn main() {
    let file_path = "src/elves.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read file");

    let lines = contents.lines();

    let mut current_elven = 0;

    let mut elvens = BinaryHeap::<i32>::new();

    lines.for_each(|line| {
        if line == "" {
            elvens.push(current_elven);

            current_elven = 0;
            return;
        }

        current_elven += line.to_string().parse::<i32>().unwrap();
    });

    let mut sum = 0;
    for _ in 0..3 {
        sum += elvens.pop().unwrap();
    }

    println!("Top 3 elvens have {} calories", sum);
}

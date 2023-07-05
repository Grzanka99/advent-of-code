use std::fs;

fn main() {
    let file = fs::read_to_string("vals").expect("Should read file");
    let lines = file.lines();

    let mut fully_contained_sections = 0;

    for line in lines {
        let (left, right) = line.split_once(',').unwrap();

        let (r1start, r1stop) = left
            .split_once('-')
            .map(|(e1, e2)| -> (i32, i32) { (e1.parse().unwrap(), e2.parse().unwrap()) })
            .unwrap();
        let (r2start, r2stop) = right
            .split_once('-')
            .map(|(e1, e2)| -> (i32, i32) { (e1.parse().unwrap(), e2.parse().unwrap()) })
            .unwrap();

        let range1 = (r1start..=r1stop).collect::<Vec<i32>>();
        let range2 = (r2start..=r2stop).collect::<Vec<i32>>();

        if range1.iter().all(|item| range2.contains(item))
            || range2.iter().all(|item| range1.contains(item))
        {
            fully_contained_sections += 1;
        }
    }

    println!("{}", fully_contained_sections);
}

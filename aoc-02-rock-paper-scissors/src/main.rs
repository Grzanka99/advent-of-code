use std::{fs, str::Lines};

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn get_correct_shape(expected_result: &str, val: &Shape) -> Option<Shape> {
        match expected_result {
            "Y" => match val {
                Shape::Rock => Some(Shape::Rock),
                Shape::Paper => Some(Shape::Paper),
                Shape::Scissors => Some(Shape::Scissors),
            },
            "Z" => match val {
                Shape::Rock => Some(Shape::Paper),
                Shape::Paper => Some(Shape::Scissors),
                Shape::Scissors => Some(Shape::Rock),
            },
            "X" => match val {
                Shape::Rock => Some(Shape::Scissors),
                Shape::Paper => Some(Shape::Rock),
                Shape::Scissors => Some(Shape::Paper),
            },
            _ => None,
        }
    }

    fn get_shape(val: &str) -> Option<Shape> {
        match val {
            "A" => Some(Shape::Rock),
            "B" => Some(Shape::Paper),
            "C" => Some(Shape::Scissors),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum Score {
    Win(usize),
    Lose(usize),
    Draw(usize),
}

impl Score {
    fn value_of(&self) -> usize {
        match self {
            Score::Win(value) => *value,
            Score::Lose(value) => *value,
            Score::Draw(value) => *value,
        }
    }
}

fn find_tokens(lines: Lines) -> Vec<(Shape, Shape)> {
    let mut tokens: Vec<(Shape, Shape)> = vec![];

    lines.for_each(|line| {
        let opponent = Shape::get_shape(line.split_whitespace().next().unwrap());
        let player = Shape::get_correct_shape(
            line.split_whitespace().last().unwrap(),
            &opponent.as_ref().unwrap(),
        );

        tokens.push((opponent.unwrap(), player.unwrap()));
    });

    return tokens;
}

fn main() {
    let content = fs::read_to_string("vals.txt").expect("Should be able to read file");
    let lines = content.lines();
    let tokens = find_tokens(lines);

    let mut scores: Vec<Score> = vec![];
    for token in tokens {
        let score = match token {
            (Shape::Rock, _) => match token {
                (_, Shape::Paper) => Score::Win(6 + 2),
                (_, Shape::Scissors) => Score::Lose(0 + 3),
                (_, Shape::Rock) => Score::Draw(1 + 3),
            },
            (Shape::Paper, _) => match token {
                (_, Shape::Scissors) => Score::Win(6 + 3),
                (_, Shape::Rock) => Score::Lose(0 + 1),
                (_, Shape::Paper) => Score::Draw(3 + 2),
            },
            (Shape::Scissors, _) => match token {
                (_, Shape::Rock) => Score::Win(6 + 1),
                (_, Shape::Paper) => Score::Lose(0 + 2),
                (_, Shape::Scissors) => Score::Draw(3 + 3),
            },
        };

        scores.push(score);
    }

    let sum: usize = scores.iter().map(|v| v.value_of()).sum();
    println!("{:?}", sum);
}

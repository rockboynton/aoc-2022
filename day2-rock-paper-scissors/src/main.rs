use std::env;
use std::fs;
use std::collections::HashMap;

use itertools::Itertools;

#[derive(Hash)]
#[derive(Eq)]
#[derive(PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Hash)]
struct Hand {
    shape: Shape,
    score: u8,
}

impl Hand {
    fn new(val: &str) -> Self {
        match val {
            "A" | "X" => Self {
                shape: Shape::Rock,
                score: 1,
            },
            "B" | "Y" => Self {
                shape: Shape::Paper,
                score: 2,
            },
            "C" | "Z" => Self {
                shape: Shape::Scissors,
                score: 3,
            },
            s => panic!("Unexpected shape string: {s}")
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let strategy_guide = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let winners: HashMap<Shape, Shape> = HashMap::from([
        (Shape::Rock, Shape::Paper),
        (Shape::Paper, Shape::Scissors),
        (Shape::Scissors, Shape::Rock),
    ]);

    let mut total_score: u64 = 0;
    for game in strategy_guide.lines() {
        let (opponent_hand, my_hand) = &game.split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| Hand::new(s))
            .collect_tuple()
            .unwrap();

        total_score += my_hand.score as u64 + if winners[&opponent_hand.shape] == my_hand.shape {
            6
        } else if opponent_hand.shape == my_hand.shape {
            3
        } else {
            0
        }
    }

    println!("Total score according to strategy: {total_score}");
}

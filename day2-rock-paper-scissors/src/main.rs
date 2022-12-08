use std::collections::HashMap;
use std::env;
use std::fs;

use itertools::Itertools;

#[derive(Hash, Eq, PartialEq, Clone)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
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
            s => panic!("Unexpected shape string: {s}"),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let strategy_guide =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let winners: HashMap<Shape, Shape> = HashMap::from([
        (Shape::Rock, Shape::Paper),
        (Shape::Paper, Shape::Scissors),
        (Shape::Scissors, Shape::Rock),
    ]);

    let total_score = part_1(&strategy_guide, &winners);
    println!("Total score for part 1 according to strategy: {total_score}");

    let total_score = part_2(&strategy_guide, &winners);
    println!("Total score for part 2 according to strategy: {total_score}");
}

/// For part 1, the second column is what you should play in response: X for Rock, Y for
/// Paper, and Z for Scissors.
fn part_1(strategy_guide: &str, winners: &HashMap<Shape, Shape>) -> u64 {
    let mut total_score = 0;
    for game in strategy_guide.lines() {
        let (opponent_hand, my_hand) = &game
            .split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| Hand::new(s))
            .collect_tuple()
            .unwrap();

        total_score += my_hand.score as u64
            + if winners[&opponent_hand.shape] == my_hand.shape {
                6
            } else if opponent_hand.shape == my_hand.shape {
                3
            } else {
                0
            }
    }

    total_score
}

/// For the part 2, the second column is as follows: X means you need to lose, Y means
/// you need to end the round in a draw, and Z means you need to win.
fn part_2(strategy_guide: &str, winners: &HashMap<Shape, Shape>) -> u64 {
    let losers: HashMap<Shape, Shape> = HashMap::from([
        (Shape::Paper, Shape::Rock),
        (Shape::Scissors, Shape::Paper),
        (Shape::Rock, Shape::Scissors),
    ]);

    let mut total_score = 0;
    for game in strategy_guide.lines() {
        let (opponent_hand, my_hand) = &game
            .split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| Hand::new(s))
            .collect_tuple()
            .unwrap();

        let score = match my_hand.shape {
            // lose
            Shape::Rock => losers[&opponent_hand.shape].clone() as u64,
            // draw
            Shape::Paper => 3 + opponent_hand.score as u64,
            // win
            Shape::Scissors => 6 + winners[&opponent_hand.shape].clone() as u64,
        };
        total_score += score;
        // correct answer: 12989
    }

    // 11319 is wrong
    total_score
}

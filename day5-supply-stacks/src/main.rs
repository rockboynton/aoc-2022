use std::collections::HashMap;
use std::env;
use std::fs;

type Crate = String;

// struct Crate(String);

type Stack = Vec<Crate>;
// struct Stack {
//     crates: Vec<Crate>,
//     id: u32
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // for line in input.lines() {
    //     println!("{:?}", line);
    // }

    crates_on_top_of_each_stack(input);
}

fn crates_on_top_of_each_stack(input: String) {
    let stacks_iter = input.lines().rev().skip_while(|&line| !line.is_empty());

    let mut stacks: HashMap<u32, Stack> = stacks_iter
        .clone()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|id| (id.parse::<u32>().unwrap(), Stack::new()))
        .collect();

    // build stacks
    for line in stacks_iter.clone().skip(2) {
        let columns = format!("{:1$}", line, stacks.len() * 4)
            .chars()
            .collect::<Vec<char>>();

        for (crate_column, i) in columns.chunks(4).zip(1u32..) {
            if !crate_column.iter().all(|c| c.is_whitespace()) {
                // println!("line {line}\n column {i}\n val {}\n", crate_column.iter().collect::<String>());
                stacks
                    .get_mut(&(i as u32))
                    .unwrap()
                    .push(crate_column.iter().collect());
            }
        }
    }

    // dbg!(stacks);

    let rearrangement_procedure = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            let rearrangement: Vec<&str> = line.split_whitespace().collect();
            let num_crates_to_move = rearrangement[1].parse::<u32>().unwrap();
            let origin = rearrangement[3].parse::<u32>().unwrap();
            let dest = rearrangement[5].parse::<u32>().unwrap();

            (num_crates_to_move, origin, dest)
        });

    let mut stacks_part_2 = stacks.clone();
    for (num_crates_to_move, origin, dest) in rearrangement_procedure {
        let mut multiple_moving_stack = Stack::new();

        for _ in 0..num_crates_to_move {
            let moving_crate_1 = stacks.get_mut(&origin).unwrap().pop().unwrap();
            let moving_crate_2 = stacks_part_2.get_mut(&origin).unwrap().pop().unwrap();
            stacks.get_mut(&dest).unwrap().push(moving_crate_1.clone());
            multiple_moving_stack.push(moving_crate_2);
        }

        for _ in 0..num_crates_to_move {
            let moving_crate_2 = multiple_moving_stack.pop().unwrap();
            stacks_part_2.get_mut(&dest).unwrap().push(moving_crate_2);
        }
    }

    println!("{}", top_of_stack_string(stacks));
    println!("{}", top_of_stack_string(stacks_part_2));
}

fn top_of_stack_string(mut stacks: HashMap<u32, Vec<String>>) -> String {
    (1..stacks.len() as u32 + 1)
        .map(|i| stacks.get_mut(&i).unwrap().pop().unwrap())
        .map(|crate_str| {
            crate_str
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<String>()
        })
        .collect()
}

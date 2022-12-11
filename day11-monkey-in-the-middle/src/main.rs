use std::env;
use std::fs;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Item {
    worry_level: u64
}

#[derive(Debug, Clone)]
enum MonkeyOp {
    Add(u64),
    Mul(u64),
    Square,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<Item>,
    operation: MonkeyOp,
    divisibility_test_val: u64,
    next_monkey_idx_if_true: usize,
    next_monkey_idx_if_false: usize,
    num_inspected_items: u64
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let notes =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let (part1, part2) = solve(&notes);
    println!("solution to part 1: {part1}");
    println!("solution to part 2: {part2}");
}

fn solve(notes: &str) -> (u64, u64) {
    let mut monkeys = Vec::new();
    for monkey_input in notes.split("\n\n") {
        let monkey_input = monkey_input.lines().collect::<Vec<&str>>();

        let starting_items = monkey_input[1].split(": ")
                .nth(1).unwrap()
                .split(", ")
                .map(|item| Item { worry_level: item.parse::<u64>().unwrap() })
                .collect::<Vec<Item>>();

        let (operator, operand) = monkey_input[2].split("old ")
            .nth(1).unwrap()
            .split(' ')
            .collect_tuple().unwrap();

        let operand = operand.parse::<u64>();

        let operation = match operator {
            "*" if operand.is_ok() => MonkeyOp::Mul(operand.unwrap()),
            "*" if operand.is_err() => MonkeyOp::Square,
            "+" => MonkeyOp::Add(operand.unwrap()),
            _ => unreachable!()
        };

        let divisibility_test_val = monkey_input[3].split("by ")
            .nth(1).unwrap()
            .parse::<u64>().unwrap();

        let next_monkey_idx_if_true = monkey_input[4].split("monkey ")
            .nth(1).unwrap()
            .parse::<usize>().unwrap();

        let next_monkey_idx_if_false = monkey_input[5].split("monkey ")
            .nth(1).unwrap()
            .parse::<usize>().unwrap();

        monkeys.push(Monkey {
            items: starting_items,
            operation,
            divisibility_test_val,
            next_monkey_idx_if_true,
            next_monkey_idx_if_false,
            num_inspected_items: 0,
        });
    }
    // println!("{monkeys:#?}");
    let mut monkeys_part2 = monkeys.clone();

    let p1 = solve_general(&mut monkeys, 20, true);
    let p2 = solve_general(&mut monkeys_part2, 10000, false);

    (p1, p2)
}

fn solve_general(monkeys: &mut Vec<Monkey>, num_rounds: usize, get_relieved: bool) -> u64 {
    let common_multiple: u64 = monkeys.iter().map(|monkey| monkey.divisibility_test_val).product();
    for round in 1..=num_rounds {
        for monkey_idx in 0..monkeys.len() {
            // println!("Monkey {monkey_idx}:");
            monkeys[monkey_idx].num_inspected_items += monkeys[monkey_idx].items.len() as u64;
            let monkey = monkeys[monkey_idx].clone();
            for item in monkey.items {
                // println!("  Monkey inspects an item with a worry level of {}.", item.worry_level);
                let mut new_worry_level = match monkeys[monkey_idx].operation {
                    MonkeyOp::Add(operand) => item.worry_level + operand,
                    MonkeyOp::Mul(operand) => item.worry_level * operand,
                    MonkeyOp::Square => item.worry_level * item.worry_level,
                };
                // println!("    Worry level is now {}.", new_worry_level);

                if get_relieved {
                    new_worry_level /= 3;
                } else {
                    new_worry_level %= common_multiple;
                }
                // println!("    Monkey gets bored with item. Worry level is divided by 3 to {}.", new_worry_level);

                let next_monkey_idx = if new_worry_level % monkeys[monkey_idx].divisibility_test_val == 0 {
                    monkey.next_monkey_idx_if_true
                } else {
                    monkey.next_monkey_idx_if_false
                };
                monkeys[next_monkey_idx].items.push(Item { worry_level: new_worry_level });
                // println!("    Item with worry level {} is thrown to monkey {}.", new_worry_level, next_monkey_idx);

            }
            monkeys[monkey_idx].items.clear();
        }

        // println!("After round {round}, the monkeys are holding items with these worry levels:");
        // for (i, monkey) in monkeys.iter().enumerate() {
            // println!("Monkey {i}: {:?}", monkey.items.iter().map(|item| item.worry_level).collect::<Vec<u64>>());
        // }

        // if round == 1 || round == 20 || round % 1000 == 0 {
        //     println!("== After round {round} ==");
        //     for (i, monkey) in monkeys.iter().enumerate() {
        //         println!("Monkey {i} inspected items {} times.", monkey.num_inspected_items);
        //     }
        // }
    }

    // for (i, monkey) in monkeys.iter().enumerate() {
        // println!("Monkey {i} inspected items {} times.", monkey.num_inspected_items);
    // }

    monkeys.sort_by(|a, b| b.num_inspected_items.partial_cmp(&a.num_inspected_items).unwrap());

    monkeys.iter()
        .take(2)
        .map(|monkey| monkey.num_inspected_items)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let input = include_str!("../example.txt");

        let (part1, _) = solve(&input.to_string());
        assert_eq!(part1, 10605);
    }

    #[test]
    fn test_solve_part2() {
        let input = include_str!("../example.txt");

        let (_, part2) = solve(&input.to_string());
        assert_eq!(part2, 2713310158);
    }
}

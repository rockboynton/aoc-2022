use std::cmp::Ordering;
use std::env;
use std::fs;
use itertools::Itertools;
use serde_json::{json, Value};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let input =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let part1 = solve(&input);
    let part2 = solve2(&input);
    println!("solution to part 1: {part1}");
    println!("solution to part 2: {part2}");
}

fn compare(lhs: &Value, rhs: &Value) -> Option<Ordering> {
    match (lhs, rhs) {
        (Value::Number(a), Value::Number(b)) => match a.as_u64().cmp(&b.as_u64()) {
            Ordering::Equal => None,
            order => Some(order),
        },
        (Value::Array(a), Value::Array(b)) => {
            if a.is_empty() || b.is_empty() {
                match a.len().cmp(&b.len()) {
                    Ordering::Equal => None,
                    order => Some(order),
                }
            } else if let Some(v) = compare(&a[0], &b[0]) {
                Some(v)
            } else {
                compare(&json!(a[1..]), &json!(b[1..]))
            }
        }
        (Value::Number(a), Value::Array(b)) => compare(&json!(vec![a]), &json!(b)),
        (Value::Array(a), Value::Number(b)) => compare(&json!(a), &json!(vec![b])),
        _ => unreachable!(),
    }
}

fn solve(input: &str) -> usize {
    input.split("\n\n")
        .map(|line_pair| {
            line_pair.lines()
                .map(|line| serde_json::from_str(line).unwrap())
                .collect_tuple::<(Value, Value)>().unwrap()
        })
        .map(|(l, r)| compare(&l, &r))
        .enumerate()
        .filter(|(_, p)| p.is_some() && matches!(p.unwrap(), Ordering::Less))
        .map(|(i, _)| i + 1)
        .sum::<usize>()
    // packets.extend([json!([[2]]), json!([[6]])]);
    // packets.sort_by(|a, b| compare(a, b).unwrap());

    // let dp1 = packets.iter().position(|p| *p == json!([[2]])).unwrap() + 1;
    // let dp2 = packets.iter().position(|p| *p == json!([[6]])).unwrap() + 1;
    // println!("p2: {:?}", dp1 * dp2);
}

fn solve2(input: &str) -> usize {
    let sorted_packets = input.lines()
        .chain(vec!["[[2]]", "[[6]]"].into_iter())
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str(line).unwrap())
        .sorted_by(|l, r| compare(l, r).unwrap())
        .collect::<Vec<_>>();

    let d1 = sorted_packets.iter().position(|p| *p == json!([[2]])).unwrap() + 1;
    let d2 = sorted_packets.iter().position(|p| *p == json!([[6]])).unwrap() + 1;

    d1 * d2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let input = include_str!("../example.txt");

        let part1 = solve(&input.to_string());
        assert_eq!(part1, 13);
    }

    #[test]
    fn test_solve_part2() {
        let input = include_str!("../example.txt");
        let part2 = solve2(&input.to_string());
        assert_eq!(part2, 140);
    }
}

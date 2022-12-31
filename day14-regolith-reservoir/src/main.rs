use core::panic;
use std::env;
use std::fs;
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl From<&str> for Point {
    /// Takes a string in the form of "x,y" and returns a Point
    /// This panics if the string is not in the proper form
    fn from(value: &str) -> Self {
        let (x, y) = value.split(',').collect_tuple().unwrap();
        Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

impl Point {
    pub fn in_abyss(&self, lower_bound: i32) -> bool {
        self.y >= lower_bound
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let input =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let part1 = solve(&input);
    // let part2 = solve2(&input);
    println!("solution to part 1: {part1}");
    // println!("solution to part 2: {part2}");
}

fn solve(input: &str) -> usize {
    // parse rock structures
    let mut rocks = HashSet::new();
    for line in input.lines() {
        let line = &line.replace(" ->", "");
        for (start, end) in line.split_whitespace().tuple_windows().map(|(x, y)| (Point::from(x), Point::from(y))) {
            if start.x == end.x {
                let bottom = i32::min(start.y, end.y);
                let top = i32::max(start.y, end.y);
                for y in bottom..=top {
                    rocks.insert(Point { x: start.x, y });
                }
            } else if start.y == end.y {
                let left = i32::min(start.x, end.x);
                let right = i32::max(start.x, end.x);
                for x in left..=right {
                    rocks.insert(Point { x, y: start.y });
                }
            } else {
                panic!("Rock formation not contiguous");
            }
        }
    }

    let lower_bound = rocks.iter().max_by(|p1, p2| p1.y.cmp(&p2.y)).unwrap().y;

    // simulate falling sand
    let origin = Point { x: 500, y: 0 };
    let mut sand_units_at_rest = HashSet::new();
    let mut sand_unit_in_motion = origin.clone();
    while !sand_unit_in_motion.in_abyss(lower_bound) {
        let point_down = Point { x: sand_unit_in_motion.x, y: sand_unit_in_motion.y + 1 };
        let point_down_left = Point { x: sand_unit_in_motion.x - 1, y: sand_unit_in_motion.y + 1 };
        let point_down_right = Point { x: sand_unit_in_motion.x + 1, y: sand_unit_in_motion.y + 1 };
        if !rocks.contains(&point_down) {
            sand_unit_in_motion = point_down;
        } else if !rocks.contains(&point_down_left) {
            sand_unit_in_motion = point_down_left;
        } else if !rocks.contains(&point_down_right) {
            sand_unit_in_motion = point_down_right;
        } else {
            sand_units_at_rest.insert(sand_unit_in_motion.clone());
            rocks.insert(sand_unit_in_motion);
            sand_unit_in_motion = origin.clone();
        }
    }

    sand_units_at_rest.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let input = include_str!("../example.txt");

        let part1 = solve(&input.to_string());
        assert_eq!(part1, 24);
    }


}

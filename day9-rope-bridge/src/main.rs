use std::env;
use std::fs;
use std::collections::HashSet;
use itertools::Itertools;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Coordinate {
    x: i32,
    y: i32
}

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

#[derive(Debug, Clone)]
struct Knot {
    coord: Coordinate,
    visited_coords: HashSet<Coordinate>,
}

impl Knot {
    fn new(start: Coordinate) -> Self {
        Self {
            coord: start,
            visited_coords: {
                let mut visited_coords = HashSet::new();
                visited_coords.insert(start);
                visited_coords
            },
        }
    }

    fn go(&mut self, direction: Direction, num_steps: i32) {
        match direction {
            Direction::Left => self.coord.x -= num_steps,
            Direction::Right => self.coord.x += num_steps,
            Direction::Up => self.coord.y += num_steps,
            Direction::Down => self.coord.y -= num_steps,
        }
    }

    fn adjacent_to(&self, other: &Knot) -> bool {
        if self.coord == other.coord {
            return true
        }

        let dx = i32::abs_diff(other.coord.x, self.coord.x);
        let dy = i32::abs_diff(other.coord.y, self.coord.y);

        (dx == 1 && dy == 0) || (dx == 0 && dy == 1) || (dx == 1 && dy == 1)
    }

    fn follow(&mut self, other: &Knot) {
        if self.adjacent_to(other) {
            return;
        }

        // If the other is ever two steps directly up, down, left, or right from the self,
        // the self must also move one step in that direction so it remains close enough
        // otherwise, move diagonally closer

        let dx = other.coord.x - self.coord.x;
        let dy = other.coord.y - self.coord.y;

        if i32::abs(dx) == 2 && dy == 0 {
            self.go(Direction::Right, dx.signum());
        } else if i32::abs(dy) == 2 && dx == 0 {
            self.go(Direction::Up, dy.signum());
        } else {
            self.go(Direction::Right, dx.signum());
            self.go(Direction::Up, dy.signum());
        }

        self.visited_coords.insert(self.coord);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let series_of_motions =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let part1 = solve_part1(&series_of_motions).unwrap();
    let part2 = solve_part2(&series_of_motions).unwrap();
    println!("solution to part 1: {part1}");
    println!("solution to part 2: {part2}");
}

fn solve_part1(series_of_motions: &str) -> Option<u32> {
    let head: Knot = Knot::new(Coordinate{ x: 0, y: 0 });
    let tails: Vec<Knot> = vec![Knot::new(Coordinate{ x: 0, y: 0 }); 1];
    solve(series_of_motions, head, tails)
}

fn solve_part2(series_of_motions: &str) -> Option<u32> {
    let head: Knot = Knot::new(Coordinate{ x: 0, y: 0 });
    let tails: Vec<Knot> = vec![Knot::new(Coordinate{ x: 0, y: 0 }); 9];
    solve(series_of_motions, head, tails)
}

fn solve(series_of_motions: &str, mut head: Knot, mut tails: Vec<Knot>) -> Option<u32> {
    for motion in series_of_motions.lines() {
        let (direction, num_steps) = motion.split_whitespace().collect_tuple()?;
        let direction = match direction {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => unreachable!()
        };
        let num_steps = num_steps.parse().unwrap();
        for _ in 0..num_steps {
            head.go(direction, 1);
            let mut leader = &head;
            for tail in tails.iter_mut() {
                tail.follow(leader);
                leader = tail;
            }
        }
    }
    Some(tails.last().unwrap().visited_coords.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        let part1 = solve_part1(&input.to_string()).unwrap();
        assert_eq!(part1, 13);
    }

    #[test]
    fn test_solve_part2() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        let part2 = solve_part2(&input.to_string()).unwrap();
        assert_eq!(part2, 1);
    }

    #[test]
    fn test_solve_part2_larger() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

        let part2 = solve_part2(&input.to_string()).unwrap();
        assert_eq!(part2, 36);
    }

    #[test]
    fn test_adjacency() {
        let knot1 = Knot::new(Coordinate { x: 0, y: 0 });
        let knot2 = Knot::new(Coordinate { x: 1, y: 1 });

        assert!(knot1.adjacent_to(&knot2));
    }
}

fn print_current_locs(head: &Knot, tail: &Knot) {
    for y in (0..6).rev() {
        for x in 0..6 {
            let c = Coordinate { x, y};
            if head.coord == c {
                print!("H");
            } else if tail.coord == c {
                print!("T");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn print_current_multiple(head: &Knot, tails: &Vec<Knot>) {
    for y in (0..6).rev() {
        for x in 0..6 {
            let c = Coordinate { x, y};
            if head.coord == c {
                print!("H");
            } else if tails.iter().any(|tail| tail.coord == c) {
                for (i, tail) in tails.iter().enumerate() {
                    if tail.coord == c {
                        print!("{i}");
                    }
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn print_visited(tail: &Knot) {
    for y in (0..6).rev() {
        for x in 0..6 {
            let c = Coordinate { x, y};
            if tail.visited_coords.iter().contains(&c) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

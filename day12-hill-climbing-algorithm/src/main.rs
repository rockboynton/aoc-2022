use std::env;
use std::fs;

use pathfinding::prelude::astar;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct HeightPos {
    x: i32,
    y: i32,
    height: char
}

impl HeightPos {
    fn new(x: i32, y: i32, heightmap: &[Vec<char>]) -> Option<Self> {
        Some(Self {
            x,
            y,
            height: *heightmap.get(x as usize)?.get(y as usize)?
        })
    }

    fn distance(&self, other: &HeightPos) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn set_height(&mut self, height: char) {
        self.height = height;
    }

    fn successors(&self, heightmap: &[Vec<char>]) -> Vec<(HeightPos, u32)> {
        let &HeightPos { x, y, .. } = self;

        vec![
            HeightPos::new(x + 1, y, heightmap),
            HeightPos::new(x, y + 1, heightmap),
            HeightPos::new(x - 1, y, heightmap),
            HeightPos::new(x, y - 1, heightmap)
        ].into_iter()
            .flatten()
            .filter(|pos| (pos.height as u32) < (self.height as u32) + 2)
            .map(|p| (p, 1)).collect()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let input =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let part1 = solve(&input, false);
    let part2 = solve(&input, true);
    println!("solution to part 1: {part1}");
    println!("solution to part 2: {part2}");
}

fn solve(input: &str, all_starts: bool) -> usize {
    let mut heightmap= Vec::new();
    let mut starts = vec![(0, 0)];
    let mut end = (0, 0);
    for (row_idx, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (col_idx, c) in line.chars().enumerate() {
            row.push(match c {
                'S' | 'a' if all_starts => {
                    starts.push((col_idx as i32, row_idx as i32));
                    'a'
                },
                'E' => {
                    end = (col_idx as i32, row_idx as i32);
                    'z'
                },
                height => height
            });
        }
        heightmap.push(row);
    }

    let starts = starts.iter().map(|start| {
        let mut height_pos = HeightPos::new(start.1, start.0, &heightmap).unwrap();
        height_pos.set_height('a');
        height_pos
    });
    let mut end = HeightPos::new(end.1, end.0, &heightmap).unwrap();
    end.set_height('z');

    starts.filter_map(|start| {
        astar(
            &start,
            |pos| pos.successors(&heightmap),
            |pos| pos.distance(&end),
            |pos| *pos == end)
    })
    .map(|path| path.1)
    .min().unwrap() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let input = include_str!("../example.txt");

        let part1 = solve(&input.to_string(), false);
        assert_eq!(part1, 31);
    }

    #[test]
    fn test_solve_part2() {
        let input = include_str!("../example.txt");

        let part1 = solve(&input.to_string(), true);
        assert_eq!(part1, 29);
    }
}

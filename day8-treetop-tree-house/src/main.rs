use std::env;
use std::fs;

type Forest = Vec<Vec<Tree>>;

#[derive(Debug, Clone, Copy)]
struct Tree {
    height: u8,
}

impl Tree {
    fn new(height: u8) -> Self {
        Self {
            height
        }
    }

    fn shorter_than(&self, other: &Self) -> bool {
        self.height < other.height
    }

    fn same_height_as(&self, other: &Self) -> bool {
        self.height == other.height
    }

    fn visible(&self, row: usize, col: usize, forest: &Forest) -> bool{
        // shorter than all from left
        forest.get(row).unwrap().iter()
            .take(col)
            .all(|tree| tree.shorter_than(self))
        // .. or all from right
        || forest.get(row).unwrap().iter().rev()
            .take(forest[0].len() - 1 - col)
            .all(|tree| tree.shorter_than(self))
        // .. or all from bottom,
        || forest.iter().rev()
            .take(forest.len() - 1 - row)
            .map(|row| row.get(col).unwrap())
            .all(|tree| tree.shorter_than(self))
        // .. or all from top
        || forest.iter()
            .take(row)
            .map(|row| row.get(col).unwrap())
            .all(|tree| tree.shorter_than(self))
    }



    fn scenic_score(&self, row: usize, col: usize, forest: &Forest) -> u32 {
        // println!("*********************");
        // dbg!(self);
        // all from left
        let mut left = forest.get(row).unwrap().iter()
            .take(col)
            .rev()
            .take_while(|tree| tree.height <= self.height)
            // .inspect(|val| println!("left {val:?}"))
            .peekable();
        // all from right
        let mut right = forest.get(row).unwrap().iter().rev()
            .take(forest[0].len() - 1 - col)
            .rev()
            // .inspect(|val| println!("right {val:?}"))
            .peekable();
        // all from bottom,
        let mut bottom = forest.iter().rev()
            .take(forest.len() - 1 - row)
            .map(|row| row.get(col).unwrap())
            .rev()
            // .take_while(|tree| tree.height <= self.height)
            // .inspect(|val| println!("bottom {val:?}"))
            .peekable();

        // all from top
        let mut top = forest.iter()
            .take(row)
            .map(|row| row.get(col).unwrap())
            .rev()
            // .take_while(|tree| tree.height <= self.height)
            // .inspect(|val| println!("top {val:?}"))
            .peekable();

        let mut left_scenic = 0;
        // if left.clone().any(|val| val.height == self.height) {
        //     left_scenic += 1;
        // }
        // left_scenic += left.take_while(|tree| tree.shorter_than(self)).count() as u32;
        while left.peek().is_some() {
            let tree = left.next().unwrap();
            if tree.shorter_than(self) {
                left_scenic += 1;
            } else if tree.same_height_as(self) || left.peek().is_none(){
                left_scenic += 1;
                break;
            }
        }
        // dbg!(left_scenic);

        let mut right_scenic = 0;
        // if right.clone().any(|val| val.height == self.height) {
        //     right_scenic += 1;
        // }
        // right_scenic += right.take_while(|tree| tree.shorter_than(self)).count() as u32;
        while right.peek().is_some() {
            let tree = right.next().unwrap();
            if tree.shorter_than(self) {
                right_scenic += 1;
            } else if tree.same_height_as(self) || right.peek().is_none() {
                right_scenic += 1;
                break;
            }
        }
        // dbg!(right_scenic);

        let mut bottom_scenic = 0;
        // if bottom.clone().any(|val| val.height == self.height) {
        //     bottom_scenic += 1;
        // }
        // bottom_scenic += bottom.take_while(|tree| tree.shorter_than(self)).count() as u32;
        while bottom.peek().is_some() {
            let tree = bottom.next().unwrap();
            if tree.shorter_than(self) {
                bottom_scenic += 1;
            } else if tree.same_height_as(self) || bottom.peek().is_none() {
                bottom_scenic += 1;
                break;
            }
        }
        // dbg!(bottom_scenic);

        let mut top_scenic = 0;
        // if top.clone().any(|val| val.height == self.height) {
        //     top_scenic += 1;
        // }
        // top_scenic += top.take_while(|tree| tree.shorter_than(self)).count() as u32;
        while top.peek().is_some() {
            let tree = top.next().unwrap();
            if tree.shorter_than(self) {
                top_scenic += 1;
            } else if tree.same_height_as(self) || top.peek().is_none() {
                top_scenic += 1;
                break;
            }
        }
        // dbg!(top_scenic);

        left_scenic * right_scenic * top_scenic * bottom_scenic
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let terminal_output =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let (part1, part2) = solve(terminal_output);
    println!("solution to part 1: {part1}");
    println!("solution to part 2: {part2}");
}

fn solve(datastream_buffer: String) -> (u32, u32) {
    let forest = datastream_buffer.lines()
        .map(|line| {
            line.chars()
                .map(|c| Tree::new(c.to_digit(10).unwrap() as u8))
                .collect::<Vec<Tree>>()
        })
        .collect::<Forest>();


    let rows = forest.len();
    let cols = forest[0].len();

    let mut trees_visible_from_outside = (rows * 2) as u32 + ((cols - 2) * 2) as u32; // init with all trees on outside
    let mut max_scenic_score = 0;
    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            if forest[row][col].visible(row, col, &forest) {
                trees_visible_from_outside += 1;
            }
            let scenic_score = forest[row][col].scenic_score(row, col, &forest);
            max_scenic_score = std::cmp::max(max_scenic_score, scenic_score);
        }
    }

    (trees_visible_from_outside, max_scenic_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "30373
25512
65332
33549
35390";

        let (part1, part2) = solve(input.to_string());
        assert_eq!(21, part1);
        assert_eq!(8, part2);
    }
}


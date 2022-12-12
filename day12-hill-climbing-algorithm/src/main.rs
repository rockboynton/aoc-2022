use std::env;
use std::fs;

type Heightmap = Vec<Vec<char>>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let input =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let part1 = solve_part1(&input);
    // let part2 = solve_part2(&series_of_motions).unwrap();
    println!("solution to part 1: {part1}");
    // println!("solution to part 2: {part2}");
}

fn solve_part1(input: &str) -> i32 {
    // let heightmap = heightmap.lines()
    //     .map(|line| {
    //         line.chars()
    //             .collect::<Vec<char>>()
    //     })
    //     .collect::<Heightmap>();

    let mut start: (usize, usize);
    let mut end: (usize, usize);

    let heightmap = Heightmap::new();
    for (y, row) in input.lines().enumerate() {
        let row = heightmap[y];
        for (x, height) in row.chars().enumerate() {
            match height {
                'S' => start = (y, x),
                'E' => end = (y, x),
                _ => ()
            }
            if height == 'S' {
                start = (y, x);
            }
        }
    }
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let input = include_str!("../example.txt");

        let part1 = solve_part1(&input.to_string());
        assert_eq!(part1, 13140);
    }
}

use std::collections::HashMap;
use std::env;
use std::fs;
use std::collections::hash_map::Entry::Occupied;

#[derive(Clone)]
enum Instruction {
    Add(i32),
    Noop
}

impl Instruction {
    fn new(line: &str) -> Self {
        match line {
            line if line.starts_with("addx") => Instruction::Add(line.split_whitespace().last().unwrap().parse().unwrap()),
            line if line.starts_with("noop") => Instruction::Noop,
            _ => unreachable!()
        }
    }

    fn cycles_to_complete(&self) -> u32 {
        match self {
            Instruction::Add(_) => 2,
            Instruction::Noop => 1,
        }
    }
}

struct Program {
    instructions: Vec<Instruction>,
    current_instruction_idx: usize,
    signal_strengths_map: HashMap<u32, i32>,
    cycle_count: u32,
    reg_x: i32,
    pixels: Vec<Vec<char>>
}

impl Program {
    fn new(cpu_instruction: &str) -> Self {
        Self {
            instructions: cpu_instruction.lines().map(|line| Instruction::new(line)).collect(),
            signal_strengths_map: HashMap::from([
                (20, 0),
                (60, 0),
                (100, 0),
                (140, 0),
                (180, 0),
                (220, 0),
            ]),
            current_instruction_idx: 0,
            cycle_count: 0,
            reg_x: 1,
            pixels: vec![vec!['.'; 40]; 6]
        }
    }

    fn check_signal(&mut self) {
        if let Occupied(mut entry) = self.signal_strengths_map.entry(self.cycle_count) {
            entry.insert(self.cycle_count as i32 * self.reg_x);
        }
    }

    fn print_sprite(&self) {
        for i in 0..self.pixels[0].len() {
            if (-1..self.reg_x + 1).contains(&(i as i32)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    fn excecute_next_instruction(&mut self) {
        // self.print_sprite();
        match self.instructions[self.current_instruction_idx] {
            Instruction::Add(val) => {
                self.set_pixel();
                self.cycle_count += 1;
                self.check_signal();
                self.set_pixel();
                self.cycle_count += 1;
                self.check_signal();
                self.reg_x += val;
            },
            Instruction::Noop => {
                self.set_pixel();
                self.cycle_count += 1;
                self.check_signal();
            },
        }

        self.current_instruction_idx += 1;
    }

    fn set_pixel(&mut self) {
        let col_idx = self.cycle_count % 40;
        let row_idx = self.cycle_count / 40;
        if (self.reg_x - 1..=self.reg_x + 1).contains(&(col_idx as i32)) {
            self.pixels[row_idx as usize][col_idx as usize] = '#';
        }
    }

    fn run(&mut self) {
        for _ in self.instructions.clone() {
            self.excecute_next_instruction();
        }
    }

    fn render_screen(&self) {
        for row in &self.pixels {
            for pixel in row {
                print!("{pixel}");
            }
            println!();
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let cpu_instructions =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let part1 = solve_part1(&cpu_instructions);
    // let part2 = solve_part2(&series_of_motions).unwrap();
    println!("solution to part 1: {part1}");
    // println!("solution to part 2: {part2}");
}

fn solve_part1(cpu_instructions: &str) -> i32 {
    let mut program = Program::new(cpu_instructions);

    program.run();
    program.render_screen();

    program.signal_strengths_map.values().sum()
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

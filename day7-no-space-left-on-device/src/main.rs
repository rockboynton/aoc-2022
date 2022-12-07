use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::BufRead;

type Arg = String;
type Output = String;

enum CdArg {
    Out,
    In(String),
    Root,
}

enum Command {
    Cd(CdArg),
    Ls,
}

struct File {
    size: u32,
}

struct Dir {
    sub_dirs: HashMap<String, Dir>,
    files: Vec<File>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let terminal_output = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("solution to part 1: {}", sum_of_dirs_gt_1000(terminal_output));
    // println!("solution to part 2: {}", chars_to_end_of_marker(datastream_buffer, 14));
}

fn sum_of_dirs_gt_1000(datastream_buffer: String) -> u32 {
    let mut iter = datastream_buffer.lines().peekable();
    while iter.peek().is_some() {
        match iter.next()
        .unwrap().read_line(buf) {

            _ => todo!(),
        }
    }
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chars_to_end_of_packet_marker() {
        asser
    }
}


use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::rc::{Rc, Weak};

const SPACE_NEEDED: u32 = 30000000;
const SPACE_TOTAL: u32 = 70000000;

#[derive(Debug, Clone)]
struct File {
    size: u32,
}

impl File {
    fn new(size: u32) -> Self {
        Self { size }
    }
}

#[derive(Debug, Clone)]
struct Dir {
    parent: Option<Weak<RefCell<Dir>>>,
    sub_dirs: HashMap<String, Rc<RefCell<Dir>>>,
    files: Vec<File>,
}

impl Dir {
    fn new(parent: Option<Weak<RefCell<Dir>>>) -> Self {
        Self {
            parent,
            sub_dirs: HashMap::new(),
            // sub_dirs: Vec::new(),
            files: Vec::new(),
        }
    }
}

struct Tree {
    root: Rc<RefCell<Dir>>,
}

impl Tree {
    fn get_all_sub_dir_sizes(
        &self,
        cwd: Option<Rc<RefCell<Dir>>>,
        all_sizes: &mut Vec<u32>,
    ) -> u32 {
        let cwd = cwd.unwrap_or_else(|| self.root.clone());
        let mut sum_dirs = 0;
        let sum_file_sizes = cwd.borrow().files.iter().map(|file| file.size).sum::<u32>();

        for dir in cwd.borrow().sub_dirs.values() {
            sum_dirs += self.get_all_sub_dir_sizes(Some(dir.clone()), all_sizes);
        }

        let total = sum_file_sizes + sum_dirs;
        all_sizes.push(total);
        total
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
    let root_dir = Rc::new(RefCell::new(Dir::new(None)));
    let mut current_dir = root_dir.clone();

    // parse buffer
    let mut iter = datastream_buffer.lines().peekable();
    while iter.peek().is_some() {
        let line = iter.next().unwrap();
        match line.chars().take(4).collect::<String>().as_str() {
            "$ ls" => {
                // iterate until next command
                while iter.peek().is_some() {
                    if iter.peek().unwrap().starts_with('$') {
                        break;
                    }
                    let ls_output = iter.next().unwrap();

                    let mut tokens = ls_output.split_whitespace();

                    // if starts with file, add to list of dir's files
                    if ls_output.starts_with(|c: char| c.is_numeric()) {
                        let size = tokens.next().unwrap().parse::<u32>().unwrap();
                        current_dir.borrow_mut().files.push(File::new(size));
                    } else {
                        // starts with dir, add to list of subdirs
                        let dir_name = tokens.nth(1).unwrap();
                        let parent = Some(Rc::downgrade(&current_dir));
                        current_dir.borrow_mut().sub_dirs.insert(
                            dir_name.to_string(),
                            Rc::new(RefCell::new(Dir::new(parent))),
                        );
                    }
                }
            }
            "$ cd" => {
                current_dir = match line.split_whitespace().nth(2).unwrap() {
                    "/" => root_dir.clone(),
                    ".." => current_dir
                        .borrow()
                        .parent
                        .as_ref()
                        .unwrap()
                        .clone()
                        .upgrade()
                        .unwrap(),
                    sub_dir => current_dir.borrow().sub_dirs.get(sub_dir).unwrap().clone(),
                };
            }
            _ => unreachable!(),
        };
    }

    let tree = Tree { root: root_dir };
    let mut all_sizes: Vec<u32> = Vec::new();
    let root_size = tree.get_all_sub_dir_sizes(None, &mut all_sizes);

    let part_1 = all_sizes
        .iter()
        .filter(|dir_size| **dir_size <= 100_000)
        .sum::<u32>();

    let unused_space = SPACE_TOTAL - root_size;
    let space_to_free = SPACE_NEEDED - unused_space;
    all_sizes.sort();
    let part_2 = all_sizes.iter().find(|dir| **dir >= space_to_free).unwrap();

    (part_1, *part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_dirs_gt_1000() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        let (part1, part2) = solve(input.to_string());
        assert_eq!(part1, 95437);
        assert_eq!(part2, 24933642);
    }
}

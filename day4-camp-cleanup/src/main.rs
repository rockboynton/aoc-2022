use std::env;
use std::fs;
use std::ops::RangeInclusive;

use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let section_assignments =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut fully_overlapping_pairs = 0;
    let mut overlapping_pairs = 0;
    for elf_pair in section_assignments.lines() {
        let (elf_1, elf_2) = elf_pair
            .split(',')
            .map(|assignment| {
                let (start, end) = assignment
                    .split('-')
                    .map(|section| section.parse::<u32>().unwrap())
                    .collect_tuple()
                    .unwrap();

                start..=end
            })
            .collect_tuple()
            .unwrap();

        if fully_overlapping(&elf_1, &elf_2) {
            fully_overlapping_pairs += 1;
        }

        if overlapping(&elf_1, &elf_2) {
            overlapping_pairs += 1;
        }
    }

    println!("Number of assignment pairs where one range fully contains the other: {fully_overlapping_pairs}");
    println!("Number of assignment pairs where one range at least partially contains the other: {overlapping_pairs}");
}

fn fully_overlapping(x: &RangeInclusive<u32>, y: &RangeInclusive<u32>) -> bool {
    (x.start() <= y.start() && x.end() >= y.end()) || (y.start() <= x.start() && y.end() >= x.end())
}

fn overlapping(x: &RangeInclusive<u32>, y: &RangeInclusive<u32>) -> bool {
    x.start() <= y.end() && y.start() <= x.end()
}

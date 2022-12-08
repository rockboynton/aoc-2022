use std::cmp;
use std::collections::BinaryHeap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let elf_inventory =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    // part 1
    let max = get_max_inventory(&elf_inventory);
    println!("Elf with the max inventory has inventory of {max}");

    // part 2
    let top_3 = get_top_3_total(&elf_inventory);
    println!("Top 3 elves have a total max inventory of {top_3}");
}

fn get_max_inventory(elf_inventory: &str) -> u32 {
    let mut max = 0;
    let mut acc = 0u32;
    for line in elf_inventory.lines() {
        if line.is_empty() {
            max = cmp::max(acc, max);
            acc = 0;
        } else {
            acc += line.parse::<u32>().unwrap();
        }
    }

    max
}

fn get_top_3_total(elf_inventory: &str) -> u32 {
    let mut heap = BinaryHeap::new();

    let mut acc = 0u32;
    for line in elf_inventory.lines() {
        if line.is_empty() {
            heap.push(acc);
            acc = 0;
        } else {
            acc += line.parse::<u32>().unwrap();
        }
    }

    // add the final acc to to the heap
    heap.push(acc);

    let mut sum = 0u32;
    for i in 0..3 {
        let max = heap.pop().expect("Not at least 3 elves");
        println!("Top {i} is {max}");
        sum += max;
    }

    sum
}

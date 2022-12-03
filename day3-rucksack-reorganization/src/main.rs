use std::env;
use std::fs;
use std::collections::HashSet;

trait Priority {
    const PRIORITY_LOWERCASE_A: u32 = 1;
    const PRIORITY_UPPERCASE_A: u32 = 27;
    fn priority(self) -> u32;
}

impl Priority for char {
    fn priority(self) -> u32 {
        if self.is_lowercase() {
            self as u32 - 'a' as u32 + <char as Priority>::PRIORITY_LOWERCASE_A
        } else {
            self as u32 - 'A' as u32 + <char as Priority>::PRIORITY_UPPERCASE_A
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let rucksacks = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    part_1(&rucksacks);
    part_2(&rucksacks);
}

fn part_1(rucksacks: &str) {
    let mut shared_items = Vec::new();
    for rucksack in rucksacks.lines() {
        let (compartment_1, compartment_2) = &rucksack.split_at(rucksack.len() / 2);

        let mut compartment_1_set = HashSet::new();
        let mut compartment_2_set = HashSet::new();
        for (c1_item, c2_item) in compartment_1.chars().zip(compartment_2.chars()) {
            compartment_1_set.insert(c1_item);
            compartment_2_set.insert(c2_item);

            if compartment_1_set.contains(&c2_item) {
                shared_items.push(c2_item.priority());
                break;
            }

            if compartment_2_set.contains(&c1_item) {
                shared_items.push(c1_item.priority());
                break;
            }
        }
    }
    println!("The sum of the priorities of the item types that appear in both compartments of each rucksack is: {}",
        shared_items.iter().sum::<u32>()
    )
}

fn part_2(rucksacks: &str) {
    let mut badges = Vec::new();
    let mut rucksack_iter = rucksacks.lines().peekable();
    while rucksack_iter.peek().is_some() {
        let elves: Vec<HashSet<char>> = vec![
            rucksack_iter.next().unwrap().chars().collect(),
            rucksack_iter.next().unwrap().chars().collect(),
            rucksack_iter.next().unwrap().chars().collect(),
        ];

        let badge = elves.into_iter().reduce(move |accum, rucksack| {
            accum
            .intersection(&rucksack)
            .copied()
            .collect()
        }).unwrap();

        badges.push(badge.iter().next().unwrap().priority());
    }
    println!("The sum of the priorities of the item type that corresponds to the badges of each three-Elf group is: {}",
        badges.iter().sum::<u32>()
    )
}

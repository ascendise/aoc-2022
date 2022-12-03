use std::{fs::File, io::{BufReader, BufRead, self}};

use crate::rucksack::Rucksack;

mod rucksack;

fn main(){
    let lines = get_puzzle_input();
    let rucksacks: Vec<Rucksack> = lines.iter().map(|l| Rucksack::new(l.as_str())).collect();
    let same_items: Vec<char> = rucksacks.iter().map(Rucksack::get_same_item).collect();
    let priority_sum: u32 = same_items.iter().map(rucksack::get_item_priority).sum();
    println!("The priority of same items in all rucksacks are {priority_sum}");

    let badges_sum: u32 = rucksack::get_badges(&lines).iter().map(rucksack::get_item_priority).sum();
    println!("The priority of all badges in the rucksacks are {badges_sum}")
}

fn get_puzzle_input() -> Vec<String> {
    let file = match File::open("puzzle-input.txt") {
        Ok(f) => f,
        Err(err) => panic!("Failed to open file with puzzle input: {err}")
    };
    BufReader::new(file).lines().collect::<Result<Vec<String>, io::Error>>().unwrap()
}
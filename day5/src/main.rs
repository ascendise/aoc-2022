use std::{fs::File, io::{BufReader, BufRead}};

use crates::{Stack, Operation};

use crate::crates::{move_crates, move_crates_9001};

mod crates;

fn main() {
    let lines = get_puzzle_input();
    let stacks = get_stacks(&lines);
    let ops = get_operations(&lines);
    let mut stacks_level1 = stacks.to_vec();
    let mut stacks_level2 = stacks.to_vec();
    for op in ops {
        stacks_level1 = move_crates(&op, &stacks_level1);
        stacks_level2 = move_crates_9001(&op, &stacks_level2);
    }
    println!("The word for level 1 is {}", get_word(&stacks_level1));
    println!("The word for level 2 is {}", get_word(&stacks_level2));
}

fn get_puzzle_input() -> Vec<String>{
    let file = match File::open("crates.txt") {
        Ok(f) => f,
        Err(err) => panic!("Failed to open file: {err}")
    };
    BufReader::new(file).lines().map(Result::unwrap).collect()
}

fn get_stacks(lines: &[String]) -> Vec<Stack> {
    crates::get_stacks(&lines[0..8], 9)
}

fn get_operations(lines: &[String]) -> Vec<Operation> {
    lines[10..].iter().map(|l| Operation::from(&l)).collect()
}

fn get_word(stacks: &[Stack]) -> String {
    let mut word = String::new();
    stacks.iter().for_each(|s| word.push(*s.last().unwrap()));
    word
}
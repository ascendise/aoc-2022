use std::{fs::File, io};

use crate::elf::{get_elves, get_highest_calories, get_total_calories_of_top_n};

mod puzzle_data_reader;
mod elf;

fn main() -> Result<(), io::Error> {
    let file = File::open("puzzle-input.txt")?;
    let lines = puzzle_data_reader::get_puzzle_data(&file);
    let mut elves = get_elves(&lines);
    println!("The highest calories are {}", get_highest_calories(&elves).unwrap());
    println!("The highest calories of top 3 are {}", get_total_calories_of_top_n(&mut elves, 3));
    Ok(())
}

use std::{fs::{File}, io::{BufReader, BufRead}};

use crate::cleanup::Assignment;

mod cleanup;

fn main() {
    let assignments = get_puzzle_input();
    let redundant_assignments = assignments.iter().filter(|a| a.is_redundant()).count();
    println!("{redundant_assignments} assignments are redundant");

    let overlapping = assignments.iter().filter(|a| a.has_overlapping_zones()).count();
    println!("{overlapping} assignments are overlapping");
}

fn get_puzzle_input() -> Vec<Assignment> {
    let file = match File::open("zones.txt") {
        Ok(f) => f,
        Err(err) => panic!("Failed to open file with zones! {err}"),
    };
    BufReader::new(file).lines().map(|l| Assignment::new(&l.unwrap()[..])).collect()
}
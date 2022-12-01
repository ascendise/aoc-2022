use std::io::{BufReader, Error};
use std::{fs::File, io::BufRead};

#[derive(Debug)]
pub enum Line {
    Value(u32),
    Empty
}

pub fn get_puzzle_data(input: &File) -> Vec<Line> {
    BufReader::new(input).lines().map(get_line).collect()
}

fn get_line(value: Result<String, Error>) -> Line {
    match value {
        Ok(v) => if v.is_empty() { Line::Empty } else { Line::Value(v.parse().unwrap()) },
        Err(error) => panic!("Failed to read file: {:?}", error)
    }
}

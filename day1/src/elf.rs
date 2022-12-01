
#[cfg(test)]
mod tests;

use crate::puzzle_data_reader::Line;

#[derive(Debug, Eq, PartialEq)]
pub struct Elf {
    calories: Vec<u32>
}

impl Elf {
    pub fn get_calories_sum(&self) -> u32 {
        self.calories.iter().sum()
    }
}

pub fn get_elves(lines: &Vec<Line>) -> Vec<Elf> {
    let split = lines.split(|l| matches!(l, Line::Empty));
    let mut elves: Vec<Elf> = vec![];
    for slice in split {
        let elf = Elf { calories: slice.into_iter().map(get_value).collect() };
        elves.push(elf);
    }
    elves
}

fn get_value(line: &Line) -> u32 {
    match line {
        Line::Value(v) => *v,
        Line::Empty => 0
    }
}

pub fn get_highest_calories(elves: &[Elf]) -> Option<u32> {
    elves.iter().map(|e| e.get_calories_sum()).max()
}

pub fn get_total_calories_of_top_n(elves: &mut [Elf], n: usize) -> u32{
    elves.sort_by(|e1, e2| e2.get_calories_sum().cmp(&e1.get_calories_sum()));
    elves.iter().take(n).map(|e| e.get_calories_sum()).sum()
}
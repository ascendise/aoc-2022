#[cfg(test)]
mod tests;

use regex::{Regex};

pub type Crate = char;
pub type Stack = Vec<Crate>;

pub fn get_stacks(lines: &[String], stack_count: usize) -> Vec<Stack> {
    let mut stacks: Vec<Stack> = vec![];
    (0..stack_count).for_each(|_| stacks.push(vec![]));
    for line in lines.iter().rev() {
        let chunks = line.as_bytes().chunks(4).collect::<Vec<&[u8]>>();
        for i in 0..stack_count {
            let chunk = String::from_utf8(chunks[i].to_vec()).unwrap();
            if !chunk.trim().is_empty() {
                stacks[i].push(get_crate(&chunk));
            } 
        }
    }
    stacks
}

fn get_crate(str: &str) -> Crate {
    let regex = Regex::new(r"\[(.?)\]").unwrap();
    let c = regex.captures(str).unwrap();
    let str = c.get(1).unwrap().as_str();
    *str.as_bytes().first().unwrap() as char
}

#[derive(Debug, PartialEq)]
pub struct Operation {
    take: usize,
    from: usize,
    to: usize
}

impl Operation {
    pub fn from(line: &str) -> Operation {
        let regex = Regex::new(r"move (?P<take>\d*) from (?P<from>\d*) to (?P<to>\d*)").unwrap();
        let captures = regex.captures(line).unwrap();
        Operation { 
            take: captures.name("take").unwrap().as_str().parse().unwrap(), 
            from: captures.name("from").unwrap().as_str().parse().unwrap(), 
            to: captures.name("to").unwrap().as_str().parse().unwrap() 
        }
    }
}

pub fn move_crates(op: &Operation, stacks: &[Stack]) -> Vec<Stack> {
    let mut stacks = stacks.to_vec();
    for _ in 0..op.take {
        let from_stack = &mut stacks[op.from-1];
        let crat = match from_stack.pop() {
            Some(c) => c,
            None => continue
        };
        let to_stack = &mut stacks[op.to-1];
        to_stack.push(crat);
    }
    stacks
}

pub fn move_crates_9001(op: &Operation, stacks: &[Stack]) -> Vec<Stack> {
    let mut stacks = stacks.to_vec();
    let stack = &mut stacks[op.from-1];
    let mut crates = stack.split_off(stack.len() - op.take);
    stacks[op.to-1].append(&mut crates);
    stacks
}
use std::vec;

use super::*;

#[test]
fn should_return_list_of_stacks_of_crates() {
    let lines = vec![
        String::from("[C]         [J]"),
        String::from("[B] [E]     [K]"),
        String::from("[A] [F] [G] [L]")
    ];
    let crates = super::get_stacks(&lines, 4);
    let expected_crates: Vec<Stack> = vec![
        vec!['A', 'B', 'C'],
        vec!['F', 'E'],
        vec!['G'],
        vec!['L', 'K', 'J']
    ];
    assert_eq!(expected_crates, crates)
}

#[test]
fn should_parse_line_to_operation() {
    let line = "move 3 from 9 to 6"; 
    let op = Operation::from(line);
    let expected_op = Operation { take: 3, from: 9, to: 6 };
    assert_eq!(expected_op, op);
}

#[test]
fn should_move_3_crates_from_1_to_3() {
    let stacks: Vec<Stack> = vec![
        vec!['A', 'B', 'C'],
        vec!['F', 'E'],
        vec!['G'],
        vec!['L', 'K', 'J']
    ];
    let operation = Operation { take: 3, from: 1, to: 3};
    let new_stack = move_crates(&operation, &stacks);
    let expected_stack: Vec<Stack> = vec![
        vec![],
        vec!['F', 'E'],
        vec!['G', 'C', 'B', 'A'],
        vec!['L', 'K', 'J']
    ]; 
    assert_eq!(expected_stack, new_stack);
}

#[test]
fn should_move_3_crates_from_1_to_3_via_9001_1() {
    let stacks: Vec<Stack> = vec![
        vec!['A', 'B', 'C'],
        vec!['F', 'E'],
        vec!['G'],
        vec!['L', 'K', 'J']
    ];
    let operation = Operation { take: 3, from: 1, to: 3};
    let new_stack = move_crates_9001(&operation, &stacks);
    let expected_stack: Vec<Stack> = vec![
        vec![],
        vec!['F', 'E'],
        vec!['G', 'A', 'B', 'C'],
        vec!['L', 'K', 'J']
    ]; 
    assert_eq!(expected_stack, new_stack);
}



#[test]
fn should_move_3_crates_from_1_to_3_via_9001_2() {
    let stacks: Vec<Stack> = vec![
        vec!['A', 'B', 'C', 'D'],
        vec!['F', 'E'],
        vec!['G'],
        vec!['L', 'K', 'J']
    ];
    let operation = Operation { take: 3, from: 1, to: 3};
    let new_stack = move_crates_9001(&operation, &stacks);
    let expected_stack: Vec<Stack> = vec![
        vec!['A'],
        vec!['F', 'E'],
        vec!['G', 'B', 'C', 'D'],
        vec!['L', 'K', 'J']
    ]; 
    assert_eq!(expected_stack, new_stack);
}
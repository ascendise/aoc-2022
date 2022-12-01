use crate::puzzle_data_reader::Line;
use super::*;

#[test]
fn should_return_elves() {
    let lines = vec![
        Line::Value(100),
        Line::Value(200),
        Line::Value(300),
        Line::Empty,
        Line::Value(400)
    ];
    let elves = super::get_elves(&lines);
    let expected_elves = vec![
        Elf { calories: vec![100, 200, 300] },
        Elf { calories: vec![400]}
    ];
    assert_eq!(expected_elves, elves);
}

#[test]
fn should_return_sum_of_elf_calories() {
    let elf = Elf { calories: vec![100, 75, 3000]};
    let calories = elf.get_calories_sum();
    let expected_calories = 3175;
    assert_eq!(expected_calories, calories);
}

#[test]
fn should_return_highest_sum_of_calories() {
    let elves = vec![
        Elf { calories: vec![100, 200, 300] },
        Elf { calories: vec![400, 200, 300] },
        Elf { calories: vec![100, 2, 300] },
        Elf { calories: vec![100, 200, 300] },
    ];
    let highest_calories = get_highest_calories(&elves).unwrap();
    let expected_highest_calories = 900;
    assert_eq!(expected_highest_calories, highest_calories);
}

#[test]
fn should_return_top_three_calories_sum() {
    let mut elves = vec![
        Elf { calories: vec![100, 200, 300] },
        Elf { calories: vec![400, 200, 300] },
        Elf { calories: vec![100, 2, 300] },
        Elf { calories: vec![100, 200, 300] },
    ];
    let total = get_total_calories_of_top_n(&mut elves, 3);
    let expected_total = 2100;
    assert_eq!(expected_total, total);
}
use super::*;

#[test]
fn should_get_rucksack_from_string() {
    let line = "abcdefgh";
    let rucksack = Rucksack::new(line); 
    let expected_rucksack = Rucksack(String::from("abcd"), String::from("efgh"));
    assert_eq!(expected_rucksack, rucksack);
}

#[test]
fn should_return_item_that_appears_in_both_compartments() {
    let rucksack = Rucksack(
        String::from("aBxdeFGH"),
        String::from("IJklxMno")
    );
    let item = rucksack.get_same_item();
    assert_eq!('x', item);
}

#[test]
fn should_return_priority_for_passed_item_1() {
    should_return_priority_for_passed_item('p', 16);
}

fn should_return_priority_for_passed_item(item: char, expected_priority: u32) {
    let priority = super::get_item_priority(&item);
    assert_eq!(expected_priority, priority);
}

#[test]
fn should_return_priority_for_passed_item_2() {
    should_return_priority_for_passed_item('L', 38);
}

#[test]
fn should_return_priority_for_passed_item_3() {
    should_return_priority_for_passed_item('P', 42);
}

#[test]
fn should_return_priority_for_passed_item_4() {
    should_return_priority_for_passed_item('v', 22);
}

#[test]
fn should_return_badges() {
    let lines = vec![
        String::from("vJrwpWtwJgWrhcsFMMfFFhFp"), 
        String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), 
        String::from("PmmdzqPrVvPwwTWBwg"),
        String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"), 
        String::from("ttgJtRGJQctTZtZT"), 
        String::from("CrZsJsPPZsGzwwsLwLmpwMDw"),
    ];
    let badges = super::get_badges(&lines);
    let expected_badges = vec!['r', 'Z'];
    assert_eq!(expected_badges, badges);
}
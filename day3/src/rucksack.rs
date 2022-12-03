#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct Rucksack(String, String);

impl Rucksack {
    pub fn new(line: &str) -> Rucksack {
        let chunks = line.split_at(line.len() / 2);
        Rucksack(String::from(chunks.0), String::from(chunks.1))
    }

    pub fn get_same_item(&self) -> char {
        for item in self.0.chars() {
            if self.1.contains(item) {
                return item;
            }
        }
        panic!("Invalid rucksack");
    }
}

pub fn get_item_priority(item: &char) -> u32 {
    if item.is_uppercase() {
        return *item as u32 - 38;
    } else {
        return *item as u32 - 96
    }
}

pub fn get_badges(lines: &Vec<String>) -> Vec<char> {
    let groups = lines.chunks(3);
    let mut badges: Vec<char> = Vec::new();
    for group in groups {
        let badge = group[0].as_bytes().iter().find(|item| has_item(item, group));
        badges.push(*badge.unwrap() as char);
    }
    badges
}

fn has_item(item: &u8, group: &[String]) -> bool {
    group.iter().all(|g| g.contains(*item as char))
}
#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct Assignment(Zone, Zone);

impl Assignment {
    pub fn new(line: &str) -> Assignment {
        let zones = split_at_remove_char(line, ',');
        Assignment(
            Zone::new(zones.0),
            Zone::new(zones.1)
        )
    }

    pub fn is_redundant(&self) -> bool {
        if self.0.is_subrange_of(&self.1) {
            return true;
        }
        self.1.is_subrange_of(&self.0)
    }

    pub fn has_overlapping_zones(&self) -> bool {
        if self.0.is_overlapping(&self.1) {
            return true;
        }
        self.1.is_overlapping(&self.0)
    }
}

#[derive(Debug, PartialEq)]
struct Zone {
    min: u32,
    max: u32,
}

impl Zone {
    fn new(line: &str) -> Zone{
        let range = split_at_remove_char(line, '-');
        Zone { 
            min: range.0.parse().unwrap(), 
            max: range.1.parse().unwrap() 
        }
    }

    fn is_subrange_of(&self, zone: &Zone) -> bool {
        self.min >= zone.min && self.max <= zone.max
    }

    fn is_overlapping(&self, zone: &Zone) -> bool {
        (self.min >= zone.min && self.min <= zone.max) ||
        (self.max >= zone.min && self.max <= zone.max )
    }
}

fn split_at_remove_char(str: &str, c: char) -> (&str, &str) {
    let index = str.find(c).unwrap();
    let mut halves = str.split_at(index);
    halves.1 = &halves.1[1..];
    halves
}
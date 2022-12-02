use crate::game::{self, Round};

#[cfg(test)]
mod tests;


#[derive(Debug, PartialEq)]
pub enum Outcome {
    Win,
    Lose,
    Draw
}

impl Outcome {
    fn get_num(&self, opponent: game::Option) -> i32 {
        match self {
            Outcome::Win => if matches!(opponent, game::Option::Scissors) { -2 } else { 1 },
            Outcome::Lose => if matches!(opponent, game::Option::Rock) { 2 } else { -1 },
            Outcome::Draw => 0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Plan {
    opponent: game::Option,
    outcome: Outcome
}

impl Plan {
    pub fn calc_round(&self) -> Round {
        let your_option = self.opponent.get_score() + self.outcome.get_num(self.opponent);
        Round {
            opponent: self.opponent,
            you: game::to_option_from_num(your_option)
        }
    }
}


pub fn get_plan(line: &str) -> Plan {
    Plan {
        opponent: game::to_option(line.as_bytes()[0] as char),
        outcome: get_outcome(line.as_bytes()[2] as char) 
    }
}

fn get_outcome(c: char) -> Outcome{
    match c {
        'X' => Outcome::Lose,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => panic!("Invalid outcome passed!")
    }
}
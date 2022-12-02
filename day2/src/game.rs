#[cfg(test)]
mod tests;

const WINNER_SCORE: i32 = 6;
const LOSS_SCORE: i32 = 0;
const DRAW_SCORE: i32 = 3;

#[derive(Debug, PartialEq)]
pub struct Round {
    pub opponent: Option,
    pub you: Option
}

impl Round {

    pub fn get_score(&self) -> i32 {
        self.you.get_score() + self.get_outcome_score()
    }

    fn get_outcome_score(&self) -> i32 {
        match self.you.get_score() - self.opponent.get_score() {
            1 | -2 => WINNER_SCORE,
            0 => DRAW_SCORE,
            _ => LOSS_SCORE
        }
    }
}

#[derive(Debug, PartialEq, Copy)]
pub enum Option {
    Rock,
    Paper,
    Scissors
}

impl Clone for Option {
    fn clone(&self) -> Self {
        match self {
            Self::Rock => Self::Rock,
            Self::Paper => Self::Paper,
            Self::Scissors => Self::Scissors,
        }
    }
}

impl Option {
    pub fn get_score(&self) -> i32{
        match self {
            Option::Rock => 1,
            Option::Paper => 2,
            Option::Scissors => 3,
        }
    }
}

pub fn to_round(line: &str) -> Round {
    return Round {
        opponent: to_option(line.as_bytes()[0] as char),
        you: to_option(line.as_bytes()[2] as char),
    }
}

pub fn to_option(c: char) -> Option {
    match c {
        'A' | 'X' => Option::Rock,
        'B' | 'Y' => Option::Paper,
        'C' | 'Z' => Option::Scissors,
        _ => panic!("Invalid option passed")
    }
}

pub fn to_option_from_num(num: i32) -> Option  {
    match num {
        1 => Option::Rock,
        2 => Option::Paper,
        3 => Option::Scissors,
        _ => panic!("Invalid option '{num}'")
    }
}

pub fn get_total_score(rounds: &[Round]) -> i32 {
    rounds.iter().map(|r| r.get_score()).sum()
}   

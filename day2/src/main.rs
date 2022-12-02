use std::{fs::File, io::{BufRead, BufReader, self}};

mod game;
mod round_calculator;

fn main() {
    let input = get_input(&get_round);
    println!("Your total score for level 1 is {}", game::get_total_score(&input));

    let calculated_rounds = get_input(&get_calculated_rounds);
    println!("Your total score for level 2 is {}", game::get_total_score(&calculated_rounds));
}

fn get_input(f: &dyn Fn(Result<String, io::Error>) -> game::Round) -> Vec<game::Round> {
    let file = match File::open("./rounds.txt") {
        Ok(f) => f,
        Err(e) => panic!("Could not read input file! {e}"),
    };
    BufReader::new(file).lines().map(f).collect()
}

fn get_round(line: Result<String, io::Error>) -> game::Round {
    let line = match line {
        Ok(l) => l,
        Err(e) => panic!("Failed to read line! {e}"),
    };
    game::to_round(&line)
} 

fn get_calculated_rounds(line: Result<String, io::Error>) -> game::Round {
    let line = match line {
        Ok(l) => l,
        Err(e) => panic!("Failed to read line! {e}"),
    };
    let plan = round_calculator::get_plan(&line);
    plan.calc_round()
} 
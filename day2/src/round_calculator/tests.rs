use super::*;

#[test]
fn should_return_plan_for_round_1() {
    let line = "A Y";
    let plan = get_plan(&line);
    let expected_plan = Plan { opponent: game::Option::Rock, outcome: Outcome::Draw };
    assert_eq!(expected_plan, plan);
}

#[test]
fn should_return_plan_for_round_2() {
    let line = "B X";
    let plan = get_plan(&line);
    let expected_plan = Plan { opponent: game::Option::Paper, outcome: Outcome::Lose };
    assert_eq!(expected_plan, plan);
}

#[test]
fn should_return_plan_for_round_3() {
    let line = "C Z";
    let plan = get_plan(&line);
    let expected_plan = Plan { opponent: game::Option::Scissors, outcome: Outcome::Win };
    assert_eq!(expected_plan, plan);
}

#[test]
fn should_return_round_for_specified_plan_1() {
    let plan = Plan { opponent: game::Option::Rock, outcome: Outcome::Draw };
    let round = plan.calc_round();
    let expected_round = game::Round { opponent: game::Option::Rock, you: game::Option::Rock };
    assert_eq!(expected_round, round);
}

#[test]
fn should_return_round_for_specified_plan_2() {
    let plan = Plan { opponent: game::Option::Paper, outcome: Outcome::Lose };
    let round = plan.calc_round();
    let expected_round = game::Round { opponent: game::Option::Paper, you: game::Option::Rock };
    assert_eq!(expected_round, round);
}

#[test]
fn should_return_round_for_specified_plan_3() {
    let plan = Plan { opponent: game::Option::Scissors, outcome: Outcome::Win };
    let round = plan.calc_round();
    let expected_round = game::Round { opponent: game::Option::Scissors, you: game::Option::Rock };
    assert_eq!(expected_round, round);
}

#[test]
fn should_return_round_for_specified_plan_4() {
    let plan = Plan { opponent: game::Option::Scissors, outcome: Outcome::Lose };
    let round = plan.calc_round();
    let expected_round = game::Round { opponent: game::Option::Scissors, you: game::Option::Paper };
    assert_eq!(expected_round, round);
}
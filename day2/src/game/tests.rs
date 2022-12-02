use super::*;

#[test]
fn should_return_round() {
    let line = "A Y";
    let round = to_round(&line);
    let expected_round = Round {
        opponent: Option::Rock,
        you: Option::Paper
    };
    assert_eq!(expected_round, round);
}

#[test]
fn should_return_score_for_outcome_and_choosing_option_1() {
    let round = Round {
        opponent: Option::Rock,
        you: Option::Paper,
    };
    let score = round.get_score();
    assert_eq!(8, score);
}

#[test]
fn should_return_score_for_outcome_and_choosing_option_2() {
    let round = Round {
        opponent: Option::Paper,
        you: Option::Rock,
    };
    let score = round.get_score();
    assert_eq!(1, score);
}

#[test]
fn should_return_score_for_outcome_and_choosing_option_3() {
    let round = Round {
        opponent: Option::Scissors,
        you: Option::Scissors,
    };
    let score = round.get_score();
    assert_eq!(6, score);
}

#[test]
fn should_return_total_score_of_all_rounds() {
    let rounds = vec![
        Round { opponent: Option::Rock, you: Option::Paper },
        Round { opponent: Option::Paper, you: Option::Rock },
        Round { opponent: Option::Scissors, you: Option::Scissors }
    ];
    let total_score = get_total_score(&rounds);
    assert_eq!(15, total_score);
}
use super::{Zone, Assignment};

#[test]
fn should_get_assignment_from_string() {
    let line = "7-7,8-42";
    let assignment = Assignment::new(&line);
    let expected_assignment = Assignment(
        Zone { min: 7, max: 7 },
        Zone { min: 8, max: 42 }
    );
    assert_eq!(expected_assignment, assignment);
}

#[test]
fn should_return_false_if_assignment_doesnt_contain_another() {
    let assignment = Assignment(
        Zone { min: 7, max: 7 },
        Zone { min: 8, max: 42 }
    );
    assert_eq!(false, assignment.is_redundant());
}


#[test]
fn should_return_true_if_assignment_contains_another_1() {
    should_return_true_if_assignment_contains_another(
        Zone { min: 2, max: 8},
        Zone { min: 3, max: 7}
    );
}

fn should_return_true_if_assignment_contains_another(zone1: Zone, zone2: Zone) {
    let assignment = Assignment(zone1, zone2);
    assert_eq!(true, assignment.is_redundant());
}

#[test]
fn should_return_true_if_assignment_contains_another_2() {
    should_return_true_if_assignment_contains_another(
        Zone { min: 2, max: 8},
        Zone { min: 2, max: 7}
    );
}

#[test]
fn should_return_false_if_zones_dont_overlap() {
    let assignment = Assignment(
        Zone { min: 1, max: 3},
        Zone { min: 4, max: 6 }
    );
    assert_eq!(false, assignment.has_overlapping_zones());
}

#[test]
fn should_return_true_if_zones_overlap() {
    let assignment = Assignment(
        Zone { min: 1, max: 3},
        Zone { min: 2, max: 5 }
    );
    assert_eq!(true, assignment.has_overlapping_zones());
}

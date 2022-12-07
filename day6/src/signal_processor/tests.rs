#[test]
fn should_return_index_of_start_of_data_1() {
    let data = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    let start_index = super::get_start_index(&data, 4);
    assert_eq!(5, start_index);
}

#[test]
fn should_return_index_of_start_of_data_2() {
    let data = "nppdvjthqldpwncqszvftbrmjlhg";
    let start_index = super::get_start_index(&data, 4);
    assert_eq!(6, start_index);
}

#[test]
fn should_return_index_of_start_of_data_3() {
    let data = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    let start_index = super::get_start_index(&data, 4);
    assert_eq!(10, start_index);
}

#[test]
fn should_return_index_of_start_of_data_4() {
    let data = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    let start_index = super::get_start_index(&data, 4);
    assert_eq!(11, start_index);
}

#[test]
fn should_return_index_of_start_of_message_1() {
    let data = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let start_index = super::get_start_index(&data, 14);
    assert_eq!(19, start_index);
}

#[test]
fn should_return_index_of_start_of_message_2() {
    let data = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    let start_index = super::get_start_index(&data, 14);
    assert_eq!(23, start_index);
}

#[test]
fn should_return_index_of_start_of_message_3() {
    let data = "nppdvjthqldpwncqszvftbrmjlhg";
    let start_index = super::get_start_index(&data, 14);
    assert_eq!(23, start_index);
}

#[test]
fn should_return_index_of_start_of_message_4() {
    let data = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    let start_index = super::get_start_index(&data, 14);
    assert_eq!(29, start_index);
}

#[test]
fn should_return_index_of_start_of_message_5() {
    let data = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    let start_index = super::get_start_index(&data, 14);
    assert_eq!(26, start_index);
}
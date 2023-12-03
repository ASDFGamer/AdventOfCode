use aoc_utils;

fn main() {
    let input = aoc_utils::load_input!();
    part2(input);
}

fn part2(input: Vec<&str>) -> String {
    todo!()
}

#[test]
fn test_input() {
    let test_input: Vec<&str> = "".split("\n").collect();
    let result = part2(test_input);
    assert_eq!(result, "".to_string())
}

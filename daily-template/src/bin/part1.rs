use aoc_utils;
use include_lines::include_lines;

fn main() {
    let input = include_lines!("{{project-name}}/res/input");
    let result = part1(&input);
    println!("Result: {}", result)
}

fn part1(input: &[&str]) -> String {
    todo!()
}

#[test]
fn test_input() {
    let result = part1(&include_lines!("{{project-name}}/res/part1_test"));
    assert_eq!(result, "".to_string())
}

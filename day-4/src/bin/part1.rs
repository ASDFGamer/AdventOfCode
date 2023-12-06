use include_lines::include_lines;

fn main() {
    let input = include_lines!("day-4/res/input");
    let result = part1(&input);
    println!("Result: {}", result)
}

fn part1(input: &[&str]) -> String {
    let mut result = 0;
    for line in input {
        let mut card_parts = line.split(":").nth(1).unwrap().split("|");
        let win_numbers: Vec<i32> = card_parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let drawn_numbers: Vec<i32> = card_parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let mut points = 0;
        for win_number in win_numbers {
            if drawn_numbers.contains(&win_number) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }
        result += points
    }
    return result.to_string();
}

#[test]
fn test_input() {
    let result = part1(&include_lines!("day-4/res/part1_test"));
    assert_eq!(result, "13".to_string())
}

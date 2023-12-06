use include_lines::include_lines;

fn main() {
    let input = include_lines!("day-4/res/input");
    let result = part2(&input);
    println!("Result: {}", result)
}

fn part2(input: &[&str]) -> String {
    let mut card_pile: Vec<i32> = vec![1; input.len()];
    for line in input {
        let mut card_parts = line.split(":").nth(1).unwrap().split("|");
        let card_number: usize = line
            .split(":")
            .next()
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
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
                points += 1;
            }
        }
        if points == 0 {
            continue;
        }
        for i in card_number + 1..card_number + 1 + points {
            card_pile[i] += card_pile[card_number];
        }
    }
    return card_pile.iter().sum::<i32>().to_string();
}

#[test]
fn test_input() {
    let result = part2(&include_lines!("day-4/res/part1_test"));
    assert_eq!(result, "30".to_string())
}

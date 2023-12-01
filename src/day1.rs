pub fn part1(input: Vec<String>){
    let mut result = 0;
    for x in input {
        let mut first_digit: Option<u32> = None;
        let mut last_digit: u32 = 0;
        for character in x.chars() {
            match character.to_digit(10) {
                None => continue,
                Some(d) => {
                    last_digit = d;
                    if first_digit.is_none() {
                        first_digit = Some(d);
                    }
                }
            }
        }
        result += last_digit;
        result += first_digit.unwrap() *10;
    }
    println!("{}", result);
}

pub fn part2(input: Vec<String>){
    let mut result = 0;
    for x in input {
        let mut first_digit: Option<u32> = None;
        let mut last_digit: u32 = 0;
        for i in 0..x.len() {
            match x.chars().nth(i).unwrap().to_digit(10) {
                None => {},
                Some(d) => {
                    last_digit = d;
                    if first_digit.is_none() {
                        first_digit = Some(d);
                    }
                    continue;
                }
            }
            let substring =  &x[i..];
            if substring.starts_with("one") {
                    last_digit = 1;
                    first_digit.get_or_insert(1);
                }
                else if substring.starts_with("two" ) {
                    last_digit = 2;
                    first_digit.get_or_insert(2);
                }
                else if substring.starts_with("three" ) {
                    last_digit = 3;
                    first_digit.get_or_insert(3);
                }
                else if substring.starts_with("four" ) {
                    last_digit = 4;
                    first_digit.get_or_insert(4);
                }
                else if substring.starts_with("five" ) {
                    last_digit = 5;
                    first_digit.get_or_insert(5);
                }
                else if substring.starts_with("six" ) {
                    last_digit = 6;
                    first_digit.get_or_insert(6);
                }
                else if substring.starts_with("seven" ) {
                    last_digit = 7;
                    first_digit.get_or_insert(7);
                }
                else if substring.starts_with("eight") {
                    last_digit = 8;
                    first_digit.get_or_insert(8);
                }
                else if substring.starts_with("nine") {
                    last_digit = 9;
                    first_digit.get_or_insert(9);
                }
                else {
                }
        }
        result += last_digit;
        result += first_digit.unwrap() *10;
    }
    println!("{}", result);
}
mod day1;
mod day2;

use crate::day2::part2;

use std::{fs::read_to_string, path::Path};

fn main() {
    println!("Start AdventOfCode");
    part2(read_lines(2, false));
}

fn read_lines(day: i8, test: bool) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let filetype: &str = if test { "test" } else { "prod" };
    let file_name = format!("{}_{}", day, filetype);
    let file_path = Path::new(".").join("res").join(file_name);
    for line in read_to_string(file_path.to_str().unwrap()).unwrap().lines() {
        result.push(line.to_string())
    }
    return result;
}

use aoc_utils::{get_dimensions, get_neighbors, Grid, GridLocation};
use include_lines::include_lines;

fn main() {
    let input = include_lines!("day-3/res/input");
    let result = part2(&input);
    println!("Result: {}", result)
}

fn part2(input: &[&str]) -> String {
    let symbols: Vec<GridLocation> = find_symbols(&input);
    let numbers: Vec<i32> = find_numbers(&input, symbols);
    let sum: i32 = numbers.iter().sum();
    return sum.to_string();
}

fn find_numbers(input: &[&str], symbols: Vec<GridLocation>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let grid = get_dimensions(input, false);
    let mut prev_locations: Vec<GridLocation> = Vec::new();
    for symbol in symbols {
        if get_element(input, &symbol) != '*' {
            continue;
        }
        let neighbors: Vec<GridLocation> = get_neighbors(&symbol, &grid);
        let mut adjacent_numbers: Vec<i32> = Vec::new();
        for neighbor in neighbors {
            if !prev_locations.contains(&neighbor) && get_element(input, &neighbor).is_digit(10) {
                let number = get_number(input, neighbor, &grid, &mut prev_locations);
                adjacent_numbers.push(number);
            }
        }
        if adjacent_numbers.len() == 2 {
            result.push(adjacent_numbers[0] * adjacent_numbers[1])
        }
    }
    return result;
}

fn get_element(grid: &[&str], location: &GridLocation) -> char {
    return grid[location.row].chars().nth(location.column).unwrap();
}

fn get_number<'a>(
    input: &[&str],
    location: GridLocation,
    grid: &Grid,
    prev_locations: &mut Vec<GridLocation>,
) -> i32 {
    let mut number: String = get_element(input, &location).to_string();
    let mut prev_location = location;
    loop {
        if prev_location.column == 0 {
            break;
        }
        prev_location = GridLocation {
            column: prev_location.column - 1,
            row: prev_location.row,
        };
        let element = get_element(input, &prev_location);
        if !element.is_digit(10) {
            break;
        }
        number = format!("{}{}", element, number);
        prev_locations.push(prev_location);
    }

    prev_location = location;
    loop {
        if prev_location.column == grid.columns - 1 {
            break;
        }
        prev_location = GridLocation {
            column: prev_location.column + 1,
            row: prev_location.row,
        };

        let element = get_element(input, &prev_location);
        if !element.is_digit(10) {
            break;
        }
        number = format!("{}{}", number, element);
        prev_locations.push(prev_location);
    }
    return number.parse::<i32>().unwrap();
}

fn find_symbols(input: &[&str]) -> Vec<GridLocation> {
    let mut result = Vec::new();
    for (row_number, row) in input.iter().enumerate() {
        for (column_number, element) in row.chars().enumerate() {
            if !(element == '.') && !element.is_digit(10) {
                result.push(GridLocation {
                    row: row_number,
                    column: column_number,
                });
            }
        }
    }
    return result;
}

#[test]
fn test_input() {
    let result = part2(&include_lines!("day-3/res/part2_test"));
    println!("Test");
    assert_eq!(result, "467835".to_string())
}

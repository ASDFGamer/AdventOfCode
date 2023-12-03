pub fn part2(input: Vec<String>) {
    let mut result = 0;
    for game in input {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        let draws = game.split(":").last().unwrap().split(";");
        for draw in draws {
            let colors = draw.split(", ");
            for color in colors {
                let count = color
                    .trim_start()
                    .split(" ")
                    .next()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
                if color.contains("red") && count > min_red {
                    min_red = count;
                } else if color.contains("green") && count > min_green {
                    min_green = count;
                } else if color.contains("blue") && count > min_blue {
                    min_blue = count;
                }
            }
        }
        result += min_red * min_green * min_blue;
    }
    println!("{}", result)
}

pub fn part1(input: Vec<String>) {
    let red = 12;
    let green = 13;
    let blue = 14;
    let mut result = 0;
    for line in input {
        let mut possible = true;
        let game_id = line
            .split(":")
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let draws = line.split(":").last().unwrap().split(";");
        for draw in draws {
            let colors = draw.split(", ");
            for color in colors {
                let count = color
                    .trim_start()
                    .split(" ")
                    .next()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
                if color.contains("red") && count > red {
                    possible = false;
                } else if color.contains("green") && count > green {
                    possible = false;
                } else if color.contains("blue") && count > blue {
                    possible = false;
                }
            }
        }
        if possible {
            result += game_id;
        }
    }
    println!("{}", result)
}

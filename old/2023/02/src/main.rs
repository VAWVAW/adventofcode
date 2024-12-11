use std::fs;

fn one(input: String) {
    fn parse_game(line: &str) -> Option<i32> {
        let line = line.strip_prefix("Game ").unwrap();
        let (id, draws) = line.split_once(':').unwrap();
        let id = i32::from_str_radix(id, 10).unwrap();

        for draw in draws.trim().split(';') {
            for color_draw in draw.split(',') {
                let (n, color) = color_draw.trim().split_once(' ').unwrap();
                let n = i32::from_str_radix(n, 10).unwrap();

                let max_n = match color {
                    "red" => 12,
                    "green" => 13,
                    "blue" => 14,
                    _ => unreachable!(),
                };

                if n > max_n {
                    return None;
                }
            }
        }
        Some(id)
    }

    let mut possible = Vec::new();

    for line in input.lines() {
        if let Some(id) = parse_game(line) {
            possible.push(id);
        }
    }
    println!("{}", possible.into_iter().sum::<i32>());
}

fn two(input: String) {
    fn parse_game(line: &str) -> u32 {
        let line = line.strip_prefix("Game ").unwrap();
        let (_id, draws) = line.split_once(':').unwrap();

        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for draw in draws.trim().split(';') {
            for color_draw in draw.split(',') {
                let (n, color) = color_draw.trim().split_once(' ').unwrap();
                let n = u32::from_str_radix(n, 10).unwrap();

                match color {
                    "red" => {
                        if n > min_red {
                            min_red = n;
                        }
                    }
                    "green" => {
                        if n > min_green {
                            min_green = n;
                        }
                    }
                    "blue" => {
                        if n > min_blue {
                            min_blue = n;
                        }
                    }
                    _ => unreachable!(),
                };
            }
        }
        min_red * min_green * min_blue
    }

    let mut powers = Vec::new();

    for line in input.lines() {
        powers.push(parse_game(line));
    }
    println!("{}", powers.into_iter().sum::<u32>());
}

fn main() {
    let (execute_first, file_name) = match std::env::args()
        .nth(1)
        .unwrap_or("test1".to_owned())
        .as_str()
    {
        "1" => (true, "data.txt"),
        "2" => (false, "data.txt"),
        "t1" => (true, "data_test1.txt"),
        "t2" => (false, "data_test2.txt"),
        _ => (true, "data_test1.txt"),
    };
    let input = fs::read_to_string(file_name).unwrap();

    if execute_first {
        one(input);
    } else {
        two(input);
    }
}

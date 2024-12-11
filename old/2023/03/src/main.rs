use std::fs;

fn one(input: String) {
    fn has_symbol(chars: &Vec<Vec<char>>, y: usize, start: usize, end: usize) -> bool {
        let can_check_left = start > 0;
        let can_check_right = end < chars[y].len();

        // check top
        if y > 0 {
            for c in chars[y - 1][start..end].iter() {
                if !c.is_ascii_digit() && c != &'.' {
                    return true;
                }
            }
            // check top left
            if can_check_left {
                let c = &chars[y - 1][start - 1];
                if !c.is_ascii_digit() && c != &'.' {
                    return true;
                }
            }
            // check top right
            if can_check_right {
                let c = &chars[y - 1][end];
                if !c.is_ascii_digit() && c != &'.' {
                    return true;
                }
            }
        }
        // check bottom
        if y < chars.len() - 1 {
            for c in chars[y + 1][start..end].iter() {
                if !c.is_ascii_digit() && c != &'.' {
                    return true;
                }
            }
            // check bottom left
            if can_check_left {
                let c = &chars[y + 1][start - 1];
                if !c.is_ascii_digit() && c != &'.' {
                    return true;
                }
            }
            // check bottom right
            if can_check_right {
                let c = &chars[y + 1][end];
                if !c.is_ascii_digit() && c != &'.' {
                    return true;
                }
            }
        }
        // check left
        if can_check_left {
            let c = &chars[y][start - 1];
            if !c.is_ascii_digit() && c != &'.' {
                return true;
            }
        }
        // check right
        if can_check_right {
            let c = &chars[y][end];
            if !c.is_ascii_digit() && c != &'.' {
                return true;
            }
        }
        false
    }
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut sum = 0;

    // check every char
    let (mut y, mut x) = (0, 0);
    while y < chars.len() {
        while x < chars[y].len() {
            // when on start of number
            if chars[y][x].is_ascii_digit() {
                // find end of number
                let mut last = x;
                while last < chars[y].len() && chars[y][last].is_ascii_digit() {
                    last += 1;
                }

                if has_symbol(&chars, y, x, last) {
                    let string: String = chars[y][x..last].iter().collect();
                    sum += u32::from_str_radix(&string, 10).unwrap();
                }

                // proceed after number
                x = last;
            }
            x += 1;
        }
        x = 0;
        y += 1;
    }

    println!("{sum}");
}

fn two(input: String) {
    fn check_number(
        chars: &Vec<Vec<char>>,
        gears: &mut Vec<Vec<(u8, Option<u32>)>>,
        y: usize,
        start: usize,
        end: usize,
    ) {
        fn check_symbol(
            chars: &Vec<Vec<char>>,
            gears: &mut Vec<Vec<(u8, Option<u32>)>>,
            y: usize,
            x: usize,
            value: u32,
        ) {
            if chars[y][x] == '*' {
                gears[y][x].0 += 1;
                gears[y][x].1 = gears[y][x].1.map(|old| old * value).or(Some(value));
            }
        }

        let can_check_left = start > 0;
        let can_check_right = end < chars[y].len();
        let string: String = chars[y][start..end].iter().collect();
        let value = u32::from_str_radix(&string, 10).unwrap();

        // check top
        if y > 0 {
            for x in start..end {
                check_symbol(chars, gears, y - 1, x, value);
            }
            // check top left
            if can_check_left {
                check_symbol(chars, gears, y - 1, start - 1, value)
            }
            // check top right
            if can_check_right {
                check_symbol(chars, gears, y - 1, end, value)
            }
        }
        // check bottom
        if y < chars.len() - 1 {
            for x in start..end {
                check_symbol(chars, gears, y + 1, x, value);
            }
            // check bottom left
            if can_check_left {
                check_symbol(chars, gears, y + 1, start - 1, value)
            }
            // check bottom right
            if can_check_right {
                check_symbol(chars, gears, y + 1, end, value)
            }
        }
        // check left
        if can_check_left {
            check_symbol(chars, gears, y, start - 1, value)
        }
        // check right
        if can_check_right {
            check_symbol(chars, gears, y, end, value)
        }
    }
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut gears: Vec<Vec<(u8, Option<u32>)>> = chars
        .iter()
        .map(|line| line.iter().map(|_| (0, None)).collect())
        .collect();

    // check every char
    let (mut y, mut x) = (0, 0);
    while y < chars.len() {
        while x < chars[y].len() {
            // when on start of number
            if chars[y][x].is_ascii_digit() {
                // find end of number
                let mut last = x;
                while last < chars[y].len() && chars[y][last].is_ascii_digit() {
                    last += 1;
                }

                check_number(&chars, &mut gears, y, x, last);

                // proceed after number
                x = last;
            }
            x += 1;
        }
        x = 0;
        y += 1;
    }

    let sum: u32 = gears
        .into_iter()
        .map(|line| {
            line.into_iter()
                .filter_map(|elem| if elem.0 != 2 { None } else { elem.1 })
                .sum::<u32>()
        })
        .sum();
    println!("{sum}");
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

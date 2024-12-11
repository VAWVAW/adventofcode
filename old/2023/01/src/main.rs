use std::fs;

fn one(input: String) {
    let mut sum = 0;
    for line in input.lines() {
        for c in line.chars(){
            if let Some(n) = c.to_digit(10) {
                sum += n * 10;
                break;
            }
        }
        for c in line.chars().rev() {
            if let Some(n) = c.to_digit(10) {
                sum += n;
                break;
            }
        }
    }
    println!("{sum}")
}

fn two(input: String) {
    fn get_digit(chars: &str) -> Option<u32> {
        if let Some(n) = chars.chars().next().unwrap().to_digit(10) {
            return Some(n);
        }
        if chars.starts_with("one") {
            return Some(1)
        }
        if chars.starts_with("two") {
            return Some(2)
        }
        if chars.starts_with("three") {
            return Some(3)
        }
        if chars.starts_with("four") {
            return Some(4)
        }
        if chars.starts_with("five") {
            return Some(5)
        }
        if chars.starts_with("six") {
            return Some(6)
        }
        if chars.starts_with("seven") {
            return Some(7)
        }
        if chars.starts_with("eight") {
            return Some(8)
        }
        if chars.starts_with("nine") {
            return Some(9)
        }
        None
    }
    let mut sum = 0;
    for line in input.lines() {
        for (i,_) in line.char_indices() {
            if let Some(n) = get_digit(line.split_at(i).1) {
                sum += n * 10;
                break;
            }
        }
        for (i,_) in line.char_indices().rev() {
            if let Some(n) = get_digit(line.split_at(i).1) {
                sum += n;
                break;
            }
        }
    }
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
        _ => (false, "data_test2.txt"),
    };
    let input = fs::read_to_string(file_name).unwrap();

    if execute_first {
        one(input);
    } else {
        two(input);
    }
}

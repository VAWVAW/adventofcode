use std::fs;

fn one(input: String) {
    let mut lines = input
        .lines()
        .map(|line| line.split_ascii_whitespace().filter(|s| s.len() > 0));
    let wins = lines
        .next()
        .unwrap()
        .zip(lines.next().unwrap())
        .skip(1)
        .map(|(time, dist)| (time.parse::<u32>().unwrap(), dist.parse::<u32>().unwrap()))
        .map(|(time, dist)| (1..time).filter(|hold| hold * (time - hold) > dist).count())
        .fold(1, |a, b| a * b);

    println!("{wins}");
}

fn two(mut input: String) {
    input.retain(|c| c.is_ascii_digit() || c == '\n');
    let mut lines = input.lines().map(|line| line.parse::<u64>().unwrap());
    let time = lines.next().unwrap();
    let dist = lines.next().unwrap();

    let wins = (1..time).filter(|hold| hold * (time - hold) > dist).count();

    println!("{wins}");
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

use std::fs;

fn one(input: String) {
    fn process_line(line: &str) -> i64 {
        let initial_values: Vec<i64> = line
            .split(' ')
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        let mut values = vec![initial_values];

        loop {
            let n = values.len() - 1;
            let n_len = values[n].len();
            if values[n].iter().all(|v| *v == 0) {
                values[n].push(0);
                break;
            }
            if n_len <= 1 {
                panic!("single value reached in pyramid");
            }

            let new_values: Vec<i64> = (0..n_len - 1)
                .map(|i| values[n][i + 1] - values[n][i])
                .collect();
            values.push(new_values);
        }
        for i in (0..values.len() - 1).rev() {
            let n = values[i].len() - 1;
            let old_val = values[i][n];
            let inc = values[i + 1][n];

            values[i].push(old_val + inc);
        }
        values[0][values[0].len() - 1]
    }
    let final_value: i64 = input.lines().map(process_line).sum();

    println!("{final_value}");
}

fn two(input: String) {
    fn process_line(line: &str) -> i64 {
        let initial_values: Vec<i64> = line
            .split(' ')
            .map(str::parse)
            .map(Result::unwrap)
            .rev()
            .collect();

        let mut values = vec![initial_values];

        loop {
            let n = values.len() - 1;
            let n_len = values[n].len();
            if values[n].iter().all(|v| *v == 0) {
                values[n].push(0);
                break;
            }
            if n_len <= 1 {
                panic!("single value reached in pyramid");
            }

            let new_values: Vec<i64> = (0..n_len - 1)
                .map(|i| values[n][i] - values[n][i + 1])
                .collect();
            values.push(new_values);
        }
        for i in (0..values.len() - 1).rev() {
            let n = values[i].len() - 1;
            let old_val = values[i][n];
            let dec = values[i + 1][n];

            values[i].push(old_val - dec);
        }
        values[0][values[0].len() - 1]
    }
    let final_value: i64 = input.lines().map(process_line).sum();

    println!("{final_value}");
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

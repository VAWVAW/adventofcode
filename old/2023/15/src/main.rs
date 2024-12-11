use std::{collections::HashMap, fs};

fn one(mut input: String) {
    input.pop();
    let sum: u32 = input
        .split(',')
        .map(|v| {
            v.as_bytes()
                .iter()
                .fold(0u8, |a, b| a.wrapping_add(*b).wrapping_mul(17))
        })
        .map(|x| x as u32)
        .sum();

    println!("{sum}");
}

fn two(mut input: String) {
    let mut boxes: HashMap<u8, Vec<(&str, u8)>> = (0..256).map(|i| (i as u8, Vec::new())).collect();
    input.pop();
    input.split(',').for_each(|v| {
        let (label, set_operation): (&str, Option<u8>) = if let Some((label, _)) = v.split_once('-')
        {
            (label, None)
        } else {
            let (label, set_str) = v.split_once('=').unwrap();
            let op: u8 = set_str.parse().unwrap();
            (label, Some(op))
        };
        let hash = label
            .as_bytes()
            .iter()
            .fold(0u8, |a, b| a.wrapping_add(*b).wrapping_mul(17));
        let cur_box = boxes.get_mut(&hash).unwrap();

        if let Some(focal) = set_operation {
            if let Some(i) = cur_box.iter().position(|v| v.0 == label) {
                cur_box.get_mut(i).unwrap().1 = focal;
            } else {
                cur_box.push((label, focal));
            }
        }
        if set_operation.is_none() {
            if let Some(i) = cur_box.iter().position(|v| v.0 == label) {
                cur_box.remove(i);
            }
        }
    });

    let sum: usize = boxes
        .iter()
        .map(|(i, lenses)| {
            lenses.iter().enumerate().fold(0, |sum, (num, lens)| {
                sum + ((*i as usize + 1) * (num + 1) * (lens.1 as usize))
            })
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

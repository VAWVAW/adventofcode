use std::fs;

fn one(input: String) {
    fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> usize {
        a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
    }

    let mut universe_map: Vec<Vec<bool>> = input
        .lines()
        .flat_map(|line| {
            let universe_line: Vec<bool> = line.chars().map(|c| c == '#').collect();

            let do_expand = universe_line.iter().all(|c| !c);

            if do_expand {
                vec![universe_line.clone(), universe_line].into_iter()
            } else {
                vec![universe_line].into_iter()
            }
        })
        .collect();

    let mut offset = 0;
    let universe_height = universe_map.len();
    let universe_width_old = universe_map[0].len();
    for x in 0..universe_width_old {
        if (0..universe_height).all(|y| !universe_map[y][x + offset]) {
            (0..universe_height).for_each(|y| universe_map[y].insert(x + offset, false));
            offset += 1;
        }
    }

    let galaxies: Vec<(usize, usize)> = universe_map
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, c)| **c)
                .map(move |(x, _)| (y, x))
        })
        .collect();

    let mut total = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            total += manhattan_distance(galaxies[i], galaxies[j]);
        }
    }

    println!("{}", total);
}

fn two(input: String, is_test: bool) {
    let universe_expansion: usize = if is_test { 10 } else { 1_000_000 };

    fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> usize {
        a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
    }

    let universe_map: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    let mut y_expansion = 0;
    let mut galaxies = Vec::new();
    for y in 0..universe_map.len() {
        if universe_map[y].iter().all(|c| !c) {
            y_expansion += universe_expansion - 1;
            continue;
        }
        let mut x_expansion = 0;
        for x in 0..universe_map[0].len() {
            if (0..universe_map.len()).all(|y| !universe_map[y][x]) {
                x_expansion += universe_expansion - 1;
                continue;
            }
            if universe_map[y][x] {
                galaxies.push((y + y_expansion, x + x_expansion));
            }
        }
    }

    let mut total = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            total += manhattan_distance(galaxies[i], galaxies[j]);
        }
    }

    println!("{}", total);
}

fn main() {
    let (execute_first, file_name, is_test) = match std::env::args()
        .nth(1)
        .unwrap_or("test1".to_owned())
        .as_str()
    {
        "1" => (true, "data.txt", false),
        "2" => (false, "data.txt", false),
        "t1" => (true, "data_test1.txt", true),
        "t2" => (false, "data_test2.txt", true),
        _ => (true, "data_test1.txt", true),
    };
    let input = fs::read_to_string(file_name).unwrap();

    if execute_first {
        one(input);
    } else {
        two(input, is_test);
    }
}

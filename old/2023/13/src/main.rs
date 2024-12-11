use std::fs;

fn one(input: String) {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Ground {
        Rock,
        Ash,
    }
    use Ground::{Ash, Rock};

    let mut patterns: Vec<Vec<Vec<Ground>>> = vec![Vec::new()];
    for line in input.lines() {
        if line == "" {
            patterns.push(Vec::new());
            continue;
        }

        let n = patterns.len() - 1;
        patterns[n].push(
            line.chars()
                .map(|c| match c {
                    '#' => Rock,
                    '.' => Ash,
                    _ => unreachable!(),
                })
                .collect(),
        )
    }

    let mut sum = 0;
    'pattern_loop: for pattern in patterns {
        // check horizontal mirror
        let height = pattern.len();
        'mirror_loop: for mirror in 1..height {
            for i in 0..mirror {
                let above = pattern.get(mirror - i - 1);
                let below = pattern.get(mirror + i);
                if above.is_some() && below.is_some() && above != below {
                    continue 'mirror_loop;
                }
            }
            sum += mirror * 100;
            continue 'pattern_loop;
        }

        // check vertical mirror
        let width = pattern[0].len();
        'mirror_loop: for mirror in 1..width {
            for i in 0..mirror {
                let Some(left_i) = mirror.checked_sub(i + 1) else {
                    break;
                };
                let right_i = mirror + i;
                if right_i < width && pattern.iter().any(|row| row[left_i] != row[right_i]) {
                    continue 'mirror_loop;
                }
            }
            sum += mirror;
            continue 'pattern_loop;
        }
        dbg!(pattern);
        panic!();
    }

    println!("{sum}");
}

fn two(input: String) {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Ground {
        Rock,
        Ash,
    }
    use Ground::{Ash, Rock};
    impl Ground {
        fn switch(&mut self) {
            *self = match self {
                Rock => Ash,
                Ash => Rock,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Mirror {
        Horizontal(usize),
        Vertical(usize),
    }
    use Mirror::{Horizontal, Vertical};

    fn get_mirrors(pattern: &Vec<Vec<Ground>>) -> Vec<Mirror> {
        let mut mirrors = Vec::new();
        // check horizontal mirror
        let height = pattern.len();
        'mirror_loop: for mirror in 1..height {
            for i in 0..mirror {
                let above = pattern.get(mirror - i - 1);
                let below = pattern.get(mirror + i);
                if above.is_some() && below.is_some() && above != below {
                    continue 'mirror_loop;
                }
            }
            mirrors.push(Horizontal(mirror));
        }

        // check vertical mirror
        let width = pattern[0].len();
        'mirror_loop: for mirror in 1..width {
            for i in 0..mirror {
                let Some(left_i) = mirror.checked_sub(i + 1) else {
                    break;
                };
                let right_i = mirror + i;
                if right_i < width && pattern.iter().any(|row| row[left_i] != row[right_i]) {
                    continue 'mirror_loop;
                }
            }
            mirrors.push(Vertical(mirror));
        }
        mirrors
    }

    let mut patterns: Vec<Vec<Vec<Ground>>> = vec![Vec::new()];
    for line in input.lines() {
        if line == "" {
            patterns.push(Vec::new());
            continue;
        }

        let n = patterns.len() - 1;
        patterns[n].push(
            line.chars()
                .map(|c| match c {
                    '#' => Rock,
                    '.' => Ash,
                    _ => unreachable!(),
                })
                .collect(),
        )
    }

    let mut sum = 0;
    'pattern_loop: for mut pattern in patterns {
        let orig_mirror = get_mirrors(&pattern)[0];
        for y in 0..pattern.len() {
            for x in 0..pattern[y].len() {
                pattern[y][x].switch();
                let mirrors = get_mirrors(&pattern);
                let mut do_continue = false;
                mirrors
                    .iter()
                    .filter(|mirror| **mirror != orig_mirror)
                    .for_each(|mirror| {
                        sum += match *mirror {
                            Horizontal(v) => 100 * v,
                            Vertical(v) => v,
                        };
                        do_continue = true;
                    });
                if do_continue {
                    continue 'pattern_loop;
                }
                pattern[y][x].switch();
            }
        }
        panic!()
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
        _ => (false, "data.txt"),
    };
    let input = fs::read_to_string(file_name).unwrap();

    if execute_first {
        one(input);
    } else {
        two(input);
    }
}

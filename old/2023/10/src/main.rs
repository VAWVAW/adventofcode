use std::{fmt::Display, fs};

fn one(input: String) {
    #[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
    struct Pipe {
        top: bool,
        right: bool,
        bottom: bool,
        left: bool,

        value: u32,
    }
    impl Display for Pipe {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if self.value > 0 && self.value < 36 {
                f.write_fmt(format_args!(
                    "{}",
                    char::from_digit(self.value, 36).unwrap()
                ))
            } else if self.top && self.right {
                f.write_str("└")
            } else if self.top && self.left {
                f.write_str("┘")
            } else if self.bottom && self.right {
                f.write_str("┌")
            } else if self.bottom && self.left {
                f.write_str("┐")
            } else if self.top && self.bottom {
                f.write_str("│")
            } else if self.left && self.right {
                f.write_str("─")
            } else {
                f.write_str(" ")
            }
        }
    }

    let mut start_x = usize::MAX;
    let mut start_y = usize::MAX;

    let mut field: Vec<Vec<Pipe>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Pipe::default(),
                    '-' => Pipe {
                        right: true,
                        left: true,
                        ..Default::default()
                    },
                    '|' => Pipe {
                        top: true,
                        bottom: true,
                        ..Default::default()
                    },
                    'F' => Pipe {
                        right: true,
                        bottom: true,
                        ..Default::default()
                    },
                    '7' => Pipe {
                        bottom: true,
                        left: true,
                        ..Default::default()
                    },
                    'J' => Pipe {
                        top: true,
                        left: true,
                        ..Default::default()
                    },
                    'L' => Pipe {
                        top: true,
                        right: true,
                        ..Default::default()
                    },
                    'S' => {
                        start_x = x;
                        start_y = y;
                        Pipe {
                            top: true,
                            right: true,
                            bottom: true,
                            left: true,
                            value: 0,
                        }
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let mut start_pipe = Pipe::default();
    if start_y > 0 && field[start_y - 1][start_x].bottom {
        start_pipe.top = true
    }
    if start_x > 0 && field[start_y][start_x - 1].right {
        start_pipe.left = true
    }
    if start_y < field.len() - 1 && field[start_y + 1][start_x].top {
        start_pipe.bottom = true
    }
    if start_x < field[start_y].len() - 1 && field[start_y][start_x + 1].left {
        start_pipe.right = true
    }
    field[start_y][start_x] = start_pipe;

    let mut current_x = start_x;
    let mut current_y = start_y;
    let mut distance = 0;
    let mut max_distance = 0;

    for mut last_direction in 0..4 {
        loop {
            let current_pipe = &mut field[current_y][current_x];
            if current_pipe.value > 0 && current_pipe.value < distance {
                max_distance = current_pipe.value + 1;
                break;
            }
            current_pipe.value = distance;

            if current_pipe.top && last_direction != 2 {
                current_y -= 1;
                last_direction = 0;
            } else if current_pipe.right && last_direction != 3 {
                current_x += 1;
                last_direction = 1;
            } else if current_pipe.bottom && last_direction != 0 {
                current_y += 1;
                last_direction = 2;
            } else if current_pipe.left && last_direction != 1 {
                current_x -= 1;
                last_direction = 3;
            }

            if current_y == start_y && current_x == start_x {
                break;
            }
            distance += 1;
        }
        current_x = start_x;
        current_y = start_y;
        distance = 0;
    }

    println!("{max_distance}");
}

fn two(input: String) {
    #[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
    struct Pipe {
        top: bool,
        right: bool,
        bottom: bool,
        left: bool,

        is_in_loop: bool,
        is_orig: bool,
        not_inside_loop: bool,
    }
    impl Display for Pipe {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if !self.not_inside_loop {
                if self.is_orig {
                    f.write_str("I")
                } else {
                    f.write_str(".")
                }
            } else if !self.is_in_loop {
                f.write_str(" ")
            } else if self.top && self.right {
                f.write_str("└")
            } else if self.top && self.left {
                f.write_str("┘")
            } else if self.bottom && self.right {
                f.write_str("┌")
            } else if self.bottom && self.left {
                f.write_str("┐")
            } else if self.top && self.bottom {
                f.write_str("│")
            } else if self.left && self.right {
                f.write_str("─")
            } else {
                f.write_str(" ")
            }
        }
    }

    let mut start_x = usize::MAX;
    let mut start_y = usize::MAX;

    let mut field: Vec<Vec<Pipe>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Pipe {
                        is_orig: true,
                        ..Default::default()
                    },
                    '-' => Pipe {
                        is_orig: true,

                        right: true,
                        left: true,
                        ..Default::default()
                    },
                    '|' => Pipe {
                        is_orig: true,

                        top: true,
                        bottom: true,
                        ..Default::default()
                    },
                    'F' => Pipe {
                        is_orig: true,

                        right: true,
                        bottom: true,
                        ..Default::default()
                    },
                    '7' => Pipe {
                        is_orig: true,

                        bottom: true,
                        left: true,
                        ..Default::default()
                    },
                    'J' => Pipe {
                        is_orig: true,

                        top: true,
                        left: true,
                        ..Default::default()
                    },
                    'L' => Pipe {
                        is_orig: true,

                        top: true,
                        right: true,
                        ..Default::default()
                    },
                    'S' => {
                        start_x = x;
                        start_y = y;
                        Pipe {
                            is_orig: true,

                            top: true,
                            right: true,
                            bottom: true,
                            left: true,

                            is_in_loop: true,
                            not_inside_loop: true,
                        }
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let mut start_pipe = Pipe {
        is_in_loop: true,
        is_orig: true,
        not_inside_loop: true,
        ..Default::default()
    };
    if start_y > 0 && field[start_y - 1][start_x].bottom {
        start_pipe.top = true
    }
    if start_x > 0 && field[start_y][start_x - 1].right {
        start_pipe.left = true
    }
    if start_y < field.len() - 1 && field[start_y + 1][start_x].top {
        start_pipe.bottom = true
    }
    if start_x < field[start_y].len() - 1 && field[start_y][start_x + 1].left {
        start_pipe.right = true
    }
    field[start_y][start_x] = start_pipe;

    let mut current_x = start_x;
    let mut current_y = start_y;
    let mut last_direction = 0;
    loop {
        let current_pipe = &mut field[current_y][current_x];
        current_pipe.is_in_loop = true;
        current_pipe.not_inside_loop = true;

        if current_pipe.top && last_direction != 2 {
            current_y -= 1;
            last_direction = 0;
        } else if current_pipe.right && last_direction != 3 {
            current_x += 1;
            last_direction = 1;
        } else if current_pipe.bottom && last_direction != 0 {
            current_y += 1;
            last_direction = 2;
        } else if current_pipe.left && last_direction != 1 {
            current_x -= 1;
            last_direction = 3;
        }

        if current_y == start_y && current_x == start_x {
            break;
        }
    }

    let mut new_field = vec![(0..field[0].len() * 2 + 1)
        .map(|_| Pipe::default())
        .collect()];
    for line in field.iter() {
        let n = new_field.len();
        new_field.push(vec![Pipe::default()]);
        new_field.push(vec![Pipe::default()]);

        for pipe in line.iter() {
            assert!(pipe.is_orig);
            new_field[n].push(*pipe);
            new_field[n].push(if pipe.is_in_loop && pipe.right {
                Pipe {
                    left: true,
                    right: true,
                    is_in_loop: true,
                    not_inside_loop: true,
                    ..Default::default()
                }
            } else {
                Pipe::default()
            });
            new_field[n + 1].push(if pipe.is_in_loop && pipe.bottom {
                Pipe {
                    top: true,
                    bottom: true,
                    is_in_loop: true,
                    not_inside_loop: true,
                    ..Default::default()
                }
            } else {
                Pipe::default()
            });
            new_field[n + 1].push(Pipe::default());
        }
    }

    let y_max = new_field.len() - 1;
    let x_max = new_field[y_max].len() - 1;
    let mut to_check = vec![(y_max, x_max)];
    while let Some((y, x)) = to_check.pop() {
        if y > 0 && !new_field[y - 1][x].not_inside_loop {
            new_field[y - 1][x].not_inside_loop = true;
            to_check.push((y - 1, x));
        }
        if x > 0 && !new_field[y][x - 1].not_inside_loop {
            new_field[y][x - 1].not_inside_loop = true;
            to_check.push((y, x - 1));
        }
        if y < y_max && !new_field[y + 1][x].not_inside_loop {
            new_field[y + 1][x].not_inside_loop = true;
            to_check.push((y + 1, x));
        }
        if x < x_max && !new_field[y][x + 1].not_inside_loop {
            new_field[y][x + 1].not_inside_loop = true;
            to_check.push((y, x + 1));
        }
    }

    let contained_fields = new_field
        .iter()
        .flat_map(|line| line.iter())
        .filter(|pipe| pipe.is_orig && !pipe.not_inside_loop)
        .count();
    println!("{contained_fields}");
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

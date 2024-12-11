use std::{collections::VecDeque, fs};

fn one(input: String) {
    #[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
    enum Level {
        #[default]
        Ground,

        Trench,
    }
    use Level::{Ground, Trench};

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Direction {
        Left,
        Right,
        Top,
        Bottom,
    }
    use Direction::{Bottom, Left, Right, Top};
    impl TryFrom<&str> for Direction {
        type Error = ();

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            match value {
                "L" => Ok(Left),
                "R" => Ok(Right),
                "U" => Ok(Top),
                "D" => Ok(Bottom),
                _ => Err(()),
            }
        }
    }

    fn area_insert(
        area: &mut VecDeque<VecDeque<Level>>,
        mut y: i32,
        mut x: i32,
        val: Level,
    ) -> (i32, i32) {
        if y >= area.len() as i32 {
            area.push_back(Default::default());
        }
        if y < 0 {
            y += 1;
            area.push_front(Default::default());
        }
        if x >= area[y as usize].len() as i32 {
            area[y as usize].resize_with(x as usize + 1, Default::default);
        }
        if x < 0 {
            x += 1;
            for line in area.iter_mut() {
                line.push_front(Default::default());
            }
        }

        area[y as usize][x as usize] = val;

        (y, x)
    }

    let mut area: VecDeque<VecDeque<Level>> = VecDeque::new();
    let mut x = 0;
    let mut y = 0;

    // process input
    for line in input.lines() {
        let mut parts = line.split(' ');

        let direction: Direction = parts.next().unwrap().try_into().unwrap();
        let count: u8 = parts.next().unwrap().parse().unwrap();

        for _ in 0..count {
            match direction {
                Left => x -= 1,
                Right => x += 1,
                Top => y -= 1,
                Bottom => y += 1,
            }
            (y, x) = area_insert(&mut area, y, x, Trench);
        }
    }

    // fill interior
    let mut to_fill = Vec::new();
    let mut x = 0;
    while x < area[1].len() - 1 {
        // Assume area has at least 2 lines and no trenches are next to each other.
        // The top row must contain a trench so the second row must contain a space that is inside
        // the pool.

        if Ground == area[1][x] {
            x += 1;
            continue;
        }
        if Trench == area[1][x + 1] {
            while Trench == area[1][x] {
                x += 1;
            }
            x += 1;
            continue;
        }
        to_fill.push((1, x + 1));
        break;
    }

    while let Some((y, x)) = to_fill.pop() {
        if Some(&Ground) == area.get(y - 1).and_then(|line| line.get(x)) {
            area[y - 1][x] = Trench;
            to_fill.push((y - 1, x));
        }
        if Some(&Ground) == area.get(y + 1).and_then(|line| line.get(x)) {
            area[y + 1][x] = Trench;
            to_fill.push((y + 1, x));
        }
        if Some(&Ground) == area.get(y).and_then(|line| line.get(x - 1)) {
            area[y][x - 1] = Trench;
            to_fill.push((y, x - 1));
        }
        if Some(&Ground) == area.get(y).and_then(|line| line.get(x + 1)) {
            area[y][x + 1] = Trench;
            to_fill.push((y, x + 1));
        }
    }

    let sum: usize = area
        .iter()
        .map(|line| line.iter().filter(|s| Trench == **s).count())
        .sum();

    println!("{sum}");
}

fn two(input: String) {
    const START_POINT: Point = Point { x: 0, y: 0 };

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Direction {
        Left,
        Right,
        Top,
        Bottom,
    }
    use Direction::{Bottom, Left, Right, Top};

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct Point {
        x: i64,
        y: i64,
    }

    // process input
    let mut border_points = 0;

    let mut area = input
        .lines()
        .scan(START_POINT, |point: &mut Point, line: &str| {
            let prev_point = *point;

            let color = line.split_once('#').unwrap().1;

            let count = i64::from_str_radix(color.get(0..5).unwrap(), 16).unwrap();
            let direction = match color.get(5..=5).unwrap() {
                "0" => Right,
                "1" => Bottom,
                "2" => Left,
                "3" => Top,
                _ => unreachable!(),
            };

            border_points += count;

            match direction {
                Right => {
                    point.x += count;
                }
                Bottom => {
                    point.y += count;
                }
                Left => point.x -= count,
                Top => {
                    point.y -= count;
                }
            }

            Some(prev_point)
        })
        // Shoelace formula
        .fold((0, START_POINT), |(sum, prev_point), point| {
            let new_area = (prev_point.y + point.y) * (prev_point.x - point.x);

            (sum + new_area, point)
        })
        .0;
    area = area / 2;

    // Pick's theorem
    let interior_points = area + 1 - border_points / 2;
    let all_points = border_points + interior_points;

    println!("{all_points}");
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

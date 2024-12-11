use std::fs;
use std::ops::{Add, AddAssign, Mul};
use std::str::FromStr;

fn one(input: String, is_test: bool) {
    let min_pos: f64 = if is_test { 7. } else { 200000000000000. };
    let max_pos: f64 = if is_test { 27. } else { 400000000000000. };

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct Vector<T> {
        x: T,
        y: T,
    }
    impl<T, U, R> Add<Vector<R>> for Vector<T>
    where
        T: Add<R, Output = U>,
    {
        type Output = Vector<U>;

        fn add(self, rhs: Vector<R>) -> Self::Output {
            Vector {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }
    impl<T> AddAssign for Vector<T>
    where
        T: AddAssign,
    {
        fn add_assign(&mut self, rhs: Self) {
            self.x += rhs.x;
            self.y += rhs.y;
        }
    }
    impl<T, U> Mul<i64> for Vector<T>
    where
        T: Mul<i64, Output = U>,
    {
        type Output = Vector<U>;

        fn mul(self, rhs: i64) -> Self::Output {
            Vector {
                x: self.x * rhs,
                y: self.y * rhs,
            }
        }
    }

    impl<T> FromStr for Vector<T>
    where
        T: FromStr,
    {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut parts = s.split(", ");
            let Ok(x) = parts.next().ok_or(())?.trim().parse() else {
                return Err(());
            };
            let Ok(y) = parts.next().ok_or(())?.trim().parse() else {
                return Err(());
            };

            Ok(Self { x, y })
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct Line {
        start: Vector<i64>,
        direction: Vector<i64>,
    }

    impl Line {
        fn intersect(self, other: Self) -> Option<Vector<f64>> {
            let p1 = self.start;
            let p2 = self.start + self.direction;
            let p3 = other.start;
            let p4 = other.start + other.direction;

            // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line_segment
            let t: f64 = ((p1.x - p3.x) * (p3.y - p4.y) - (p1.y - p3.y) * (p3.x - p4.x)) as f64
                / ((p1.x - p2.x) * (p3.y - p4.y) - (p1.y - p2.y) * (p3.x - p4.x)) as f64;

            let u: f64 = ((p1.x - p3.x) * (p1.y - p2.y) - (p1.y - p3.y) * (p1.x - p2.x)) as f64
                / ((p1.x - p2.x) * (p3.y - p4.y) - (p1.y - p2.y) * (p3.x - p4.x)) as f64;

            if t < 0. || u < 0. {
                // before start
                return None;
            }

            let x = self.start.x as f64 + self.direction.x as f64 * t;
            let y = self.start.y as f64 + self.direction.y as f64 * t;

            Some(Vector { x, y })
        }
    }

    let storms: Vec<Line> = input
        .lines()
        .map(|line| {
            let (start_s, direction_s) = line.split_once(" @ ").unwrap();
            Line {
                start: start_s.parse().unwrap(),
                direction: direction_s.parse().unwrap(),
            }
        })
        .collect();

    let mut count: u32 = 0;
    for a in 0..storms.len() {
        for b in a + 1..storms.len() {
            if let Some(intersect) = storms[a].intersect(storms[b]) {
                if min_pos <= intersect.x
                    && intersect.x <= max_pos
                    && min_pos <= intersect.y
                    && intersect.y <= max_pos
                {
                    count += 1;
                }
            }
        }
    }

    println!("{count}");
}

fn main() {
    let (execute_first, file_name, is_test) = match std::env::args()
        .nth(1)
        .unwrap_or("test1".to_owned())
        .as_str()
    {
        "1" => (true, "data.txt", false),
        "t1" => (true, "data_test1.txt", true),
        _ => (true, "data_test1.txt", true),
    };
    let input = fs::read_to_string(file_name).unwrap();

    if execute_first {
        one(input, is_test);
    } else {
        unimplemented!();
    }
}

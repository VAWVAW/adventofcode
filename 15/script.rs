use std::fs;
use std::cmp;

static MAX_VALUE: i32 = 4000000 ;
static FILE_NAME: str = "values.txt";

#[derive(Debug)]
struct Sensor{
    x: i32,
    y: i32,
    d: i32
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Range{
    left: i32,
    right: i32
}

#[derive(Debug)]
struct FullRange{
    left: i32,
    right: i32
}

impl FullRange{
    fn new() -> FullRange {
        FullRange{
            left: 0,
            right: 0
        }
    }
}

fn main(){
    let content = fs::read_to_string(FILE_NAME).expect("file not read");

    let mut sensors: Vec<Sensor> = Vec::new();
    for line in content.lines() {
        let values = line.split(':').collect::<Vec<_>>();
        let x = values[0].parse::<i32>().unwrap();
        let y = values[1].parse::<i32>().unwrap();
        let d = values[2].parse::<i32>().unwrap();
        let to_add = Sensor{x, y, d};
        sensors.push(to_add);
    }

    for y in 0..MAX_VALUE{
        let mut lr: Vec<Range> = Vec::new();
        for sensor in &sensors {
            if sensor.d < sensor.y - y {
                continue
            }

            let distance = (sensor.y -y).abs();
            let left = sensor.x - sensor.d + distance;
            let right = sensor.x + sensor.d - distance;
            lr.push(Range{ left, right});
        }
        lr.sort();
        let mut range = FullRange::new();

        for i in 0..lr.len()-1 {
            let other = &lr[i];
            if range.right < other.left - 1 {
                let x = range.right + 1;
                if x > MAX_VALUE {
                    continue;
                }
                println!("x: {x}, y: {y}")
            }
            range.right = cmp::max(range.right, other.right);
        }
    }
}
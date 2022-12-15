use std::fs;
use std::cmp;
use std::thread;
use std::sync::{mpsc, Arc};

static MAX_VALUE: i32 = 4000000;
static N_THREADS: i32 = 8;

#[derive(Debug, Clone)]
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
    right: i32
}

impl FullRange{
    fn new() -> FullRange {
        FullRange{
            right: 0
        }
    }
}

fn calc(tx: mpsc::Sender<u64>, y_min: i32, y_max: i32, sensors: Arc<Vec<Sensor>>) {
    'outer: for y in y_min..y_max {
        let mut lr: Vec<Range> = Vec::new();
        for sensor in sensors.iter() {
            if sensor.d < sensor.y - y {
                continue
            }

            let distance = (sensor.y - y).abs();
            let left = sensor.x - sensor.d + distance;
            let right = sensor.x + sensor.d - distance;
            lr.push(Range { left, right });
        }
        lr.sort();
        let mut range = FullRange::new();

        for i in 0..lr.len() - 1 {
            let other = &lr[i];
            if range.right < other.left - 1 {
                let x = range.right + 1;
                if x > MAX_VALUE {
                    continue;
                }
                let v: u64 = (x as u64) * 4000000 + (y as u64);
                tx.send(v).unwrap();
                continue 'outer;
            }
            range.right = cmp::max(range.right, other.right);
        }
    }
}

fn two(){
    let content = fs::read_to_string("data.txt").expect("file not read");

    let mut sensors_build: Vec<Sensor> = Vec::new();
    for line in content.lines() {
        let values_s = line
            .replace("Sensor at x=", "")
            .replace(", y=", ":")
            .replace(": closest beacon is at x=", ":")
            .replace(", y=", ":")
            .replace("\n", "");
        let values = values_s.split(":").collect::<Vec<_>>();
        let x_s = values[0].parse::<i32>().unwrap();
        let y_s = values[1].parse::<i32>().unwrap();
        let x_d = values[2].parse::<i32>().unwrap();
        let y_d = values[3].parse::<i32>().unwrap();

        let distance = (x_s - x_d).abs() + (y_s - y_d).abs();
        sensors_build.push(Sensor{x: x_s, y: y_s, d: distance});
    }

    let (tx, rx) = mpsc::channel();
    let sensors = Arc::new(sensors_build);

    let diff: i32 = MAX_VALUE / N_THREADS;
    let mut handles = Vec::new();
    for i in 0..N_THREADS {
        let y_min = i * diff;
        let s = Arc::clone(&sensors);
        let new_tx = tx.clone();
        handles.push(thread::spawn(move || calc(new_tx, y_min, y_min + diff, s)));
    }

    drop(tx);
    for received in rx {
        println!("{}", received);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

fn main(){
    two();
}
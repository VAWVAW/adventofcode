use std::fs;

fn one(input: String) {
    #[derive(Debug)]
    struct Map {
        maps: Vec<(u64, u64, u64)>,
    }

    impl Map {
        fn new() -> Self {
            Self { maps: Vec::new() }
        }

        fn add_map(&mut self, map: &str) {
            let mut iter = map.split(' ').map(str::parse).filter_map(Result::ok);

            self.maps.push((
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            ));
        }
        fn map_value(&self, val: u64) -> u64 {
            for map in self.maps.iter() {
                if val < map.1 || val >= map.1 + map.2 {
                    continue;
                }
                return map.0 + (val - map.1);
            }
            val
        }
    }

    let mut lines = input.lines();

    let mut seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(str::parse)
        .filter_map(Result::ok)
        .collect();

    lines.next().unwrap();

    let mut maps: Vec<Map> = vec![Map::new()];
    let mut i = 0;
    for line in lines {
        if line == "" {
            i += 1;
            maps.push(Map::new());
            continue;
        }
        if !line.chars().next().unwrap().is_ascii_digit() {
            continue;
        }
        maps[i].add_map(line);
    }

    for map in maps.iter() {
        for seed in seeds.iter_mut() {
            *seed = map.map_value(*seed);
        }
    }

    println!("{}", seeds.iter().min().unwrap());
}

fn two(input: String) {
    #[derive(Debug)]
    struct Map {
        maps: Vec<(u64, u64, u64)>,
    }

    impl Map {
        fn new() -> Self {
            Self { maps: Vec::new() }
        }

        fn add_map(&mut self, map: &str) {
            let mut iter = map.split(' ').map(str::parse).filter_map(Result::ok);

            self.maps.push((
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            ));
        }
        fn prepare(&mut self) {
            let mut end = 0;
            let mut maps = Vec::new();

            self.maps.as_mut_slice().sort_by_key(|map| map.1);

            for map in self.maps.drain(..) {
                maps.push((end, end, map.1 - end));
                maps.push(map);
                end = map.1 + map.2;
            }

            maps.push((end, end, i64::MAX as u64));

            maps.retain(|map| map.2 != 0);

            self.maps = maps;
        }
        fn map_range(&self, range: &Range) -> Vec<Range> {
            let ret = self
                .maps
                .iter()
                .filter(|map| {
                    // range start is in map
                    map.1 <= range.start && range.start <= map.1 + map.2 ||
                    // range end is in map
                    map.1 <= range.start + range.n && range.start + range.n <= map.1 + map.2 ||
                    // map is in range
                    range.start <= map.1 && map.1 + map.2 <= range.start + range.n
                })
                .map(|map| {
                    let start;
                    let n;

                    if map.1 < range.start {
                        // use range start
                        start = range.start - map.1 + map.0;
                        if range.start + range.n > map.1 + map.2 {
                            n = map.1 + map.2 - range.start
                        } else {
                            n = range.n
                        }
                    } else {
                        // use map start
                        start = map.0;
                        if range.n >= map.2 {
                            n = map.2
                        } else {
                            n = range.start + range.n - map.1
                        }
                    }

                    Range { start, n }
                })
                .collect();
            ret
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Range {
        start: u64,
        n: u64,
    }

    let mut lines = input.lines();

    let mut seeds = Vec::new();
    let mut seed_iter = lines
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(str::parse::<u64>)
        .filter_map(Result::ok);

    loop {
        let start = if let Some(s) = seed_iter.next() {
            s
        } else {
            break;
        };
        let end = if let Some(s) = seed_iter.next() {
            s
        } else {
            break;
        };
        seeds.push(Range { start, n: end });
    }

    lines.next().unwrap();

    let mut maps: Vec<Map> = vec![Map::new()];
    let mut i = 0;
    for line in lines {
        if line == "" {
            maps[i].prepare();
            i += 1;
            maps.push(Map::new());
            continue;
        }
        if !line.chars().next().unwrap().is_ascii_digit() {
            continue;
        }
        maps[i].add_map(line);
    }
    let n = maps.len();
    maps[n - 1].prepare();

    let mut new_seeds = Vec::new();
    for map in maps.iter() {
        for seed in seeds.iter_mut() {
            new_seeds.append(&mut map.map_range(seed));
        }
        seeds = new_seeds;
        new_seeds = Vec::new();
    }

    println!("{}", seeds.iter().min().unwrap().start);
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

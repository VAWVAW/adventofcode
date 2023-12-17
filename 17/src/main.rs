use std::fs;

use pathfinding::prelude::dijkstra;

fn one(input: String) {
    const MAX_IN_DIRECTION: u8 = 3;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum Direction {
        Right,
        Left,
        Top,
        Bottom,
    }
    use Direction::{Bottom, Left, Right, Top};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct Node {
        x: usize,
        y: usize,
        direction: Direction,
        traveled_in_dir: u8,
    }

    let heat_map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let max_y = heat_map.len() - 1;
    let max_x = heat_map[0].len() - 1;

    let start = Node {
        x: 0,
        y: 0,
        direction: Top,
        traveled_in_dir: 0,
    };
    let successors = |node: &Node| {
        let mut suc = Vec::with_capacity(4);
        if node.y > 0
            && !(Top == node.direction && MAX_IN_DIRECTION == node.traveled_in_dir)
            && node.direction != Bottom
        {
            let traveled_in_dir = if Top == node.direction {
                node.traveled_in_dir + 1
            } else {
                1
            };
            let new_node = Node {
                x: node.x,
                y: node.y - 1,
                direction: Top,
                traveled_in_dir,
            };
            let new_heat = heat_map[new_node.y][new_node.x] as u32;
            suc.push((new_node, new_heat));
        }
        if node.y < max_y
            && !(Bottom == node.direction && MAX_IN_DIRECTION == node.traveled_in_dir)
            && node.direction != Top
        {
            let traveled_in_dir = if Bottom == node.direction {
                node.traveled_in_dir + 1
            } else {
                1
            };
            let new_node = Node {
                x: node.x,
                y: node.y + 1,
                direction: Bottom,
                traveled_in_dir,
            };
            let new_heat = heat_map[new_node.y][new_node.x] as u32;
            suc.push((new_node, new_heat));
        }
        if node.x > 0
            && !(Left == node.direction && MAX_IN_DIRECTION == node.traveled_in_dir)
            && node.direction != Left
        {
            let traveled_in_dir = if Left == node.direction {
                node.traveled_in_dir + 1
            } else {
                1
            };
            let new_node = Node {
                x: node.x - 1,
                y: node.y,
                direction: Left,
                traveled_in_dir,
            };
            let new_heat = heat_map[new_node.y][new_node.x] as u32;
            suc.push((new_node, new_heat));
        }
        if node.x < max_x
            && !(Right == node.direction && MAX_IN_DIRECTION == node.traveled_in_dir)
            && node.direction != Left
        {
            let traveled_in_dir = if Right == node.direction {
                node.traveled_in_dir + 1
            } else {
                1
            };
            let new_node = Node {
                x: node.x + 1,
                y: node.y,
                direction: Right,
                traveled_in_dir,
            };
            let new_heat = heat_map[new_node.y][new_node.x] as u32;
            suc.push((new_node, new_heat));
        }
        suc
    };
    let success = |node: &Node| node.y == max_y && node.x == max_x;

    let (_nodes, cost) = dijkstra(&start, successors, success).expect("no way found");
    println!("{cost}");
}

fn two(input: String) {
    const MIN_IN_DIRECTION: u8 = 4;
    const MAX_IN_DIRECTION: u8 = 10;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum Direction {
        Right,
        Left,
        Top,
        Bottom,
    }
    impl Direction {
        #[inline]
        fn go_in(self, y: usize, x: usize, max_y: usize, max_x: usize) -> Option<(usize, usize)> {
            match self {
                Right => {
                    if x < max_x {
                        Some((y, x + 1))
                    } else {
                        None
                    }
                }
                Left => {
                    if x > 0 {
                        Some((y, x - 1))
                    } else {
                        None
                    }
                }
                Top => {
                    if y > 0 {
                        Some((y - 1, x))
                    } else {
                        None
                    }
                }
                Bottom => {
                    if y < max_y {
                        Some((y + 1, x))
                    } else {
                        None
                    }
                }
            }
        }
        #[inline]
        fn reverse(self) -> Self {
            match self {
                Right => Left,
                Left => Right,
                Top => Bottom,
                Bottom => Top,
            }
        }
    }
    use Direction::{Bottom, Left, Right, Top};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct Node {
        x: usize,
        y: usize,
        direction: Direction,
        traveled_in_dir: u8,
    }
    impl Node {
        fn get_in_direction(&self, dir: Direction, max_y: usize, max_x: usize) -> Option<Self> {
            // can't reverse direction
            if dir == self.direction.reverse() {
                return None;
            }
            // check if new block is in range
            let Some((new_y, new_x)) = dir.go_in(self.y, self.x, max_y, max_x) else {
                return None;
            };

            if dir == self.direction {
                if MAX_IN_DIRECTION == self.traveled_in_dir {
                    None
                } else {
                    Some(Self {
                        x: new_x,
                        y: new_y,
                        direction: dir,
                        traveled_in_dir: self.traveled_in_dir + 1,
                    })
                }
            } else {
                if MIN_IN_DIRECTION > self.traveled_in_dir {
                    None
                } else {
                    Some(Self {
                        x: new_x,
                        y: new_y,
                        direction: dir,
                        traveled_in_dir: 1,
                    })
                }
            }
        }
    }

    let heat_map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let max_y = heat_map.len() - 1;
    let max_x = heat_map[0].len() - 1;

    let start = Node {
        x: 0,
        y: 0,
        direction: Top,
        traveled_in_dir: MIN_IN_DIRECTION,
    };
    let successors = |node: &Node| {
        let mut suc = Vec::with_capacity(4);
        if let Some(new_node) = node.get_in_direction(Top, max_y, max_x) {
            let new_heat = heat_map[new_node.y][new_node.x] as u32;
            suc.push((new_node, new_heat))
        }
        if let Some(new_node) = node.get_in_direction(Bottom, max_y, max_x) {
            let new_heat = heat_map[new_node.y][new_node.x] as u32;
            suc.push((new_node, new_heat))
        }
        if let Some(new_node) = node.get_in_direction(Right, max_y, max_x) {
            let new_heat = heat_map[new_node.y][new_node.x] as u32;
            suc.push((new_node, new_heat))
        }
        if let Some(new_node) = node.get_in_direction(Left, max_y, max_x) {
            let new_heat = heat_map[new_node.y][new_node.x] as u32;
            suc.push((new_node, new_heat))
        }
        suc
    };
    let success = |node: &Node| {
        node.y == max_y && node.x == max_x && node.traveled_in_dir >= MIN_IN_DIRECTION
    };

    let (_nodes, cost) = dijkstra(&start, successors, success).expect("no way found");
    println!("{cost}");
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
        "t22" => (false, "data_test22.txt"),
        _ => (true, "data_test1.txt"),
    };
    let input = fs::read_to_string(file_name).unwrap();

    if execute_first {
        one(input);
    } else {
        two(input);
    }
}

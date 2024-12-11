use std::fmt::Display;
use std::fs;
use std::ops::{Add, AddAssign};

use petgraph::algo::all_simple_paths;
use petgraph::prelude::NodeIndex;
use petgraph::Graph;

fn one(input: String) {
    type Coordinate = i16;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Direction {
        Top,
        Bottom,
        Left,
        Right,
    }
    use Direction::{Bottom, Left, Right, Top};

    impl Into<Offset> for Direction {
        fn into(self) -> Offset {
            match self {
                Top => Offset { x: 0, y: -1 },
                Bottom => Offset { x: 0, y: 1 },
                Left => Offset { x: -1, y: 0 },
                Right => Offset { x: 1, y: 0 },
            }
        }
    }

    impl Direction {
        fn reverse(self) -> Self {
            match self {
                Top => Bottom,
                Bottom => Top,
                Left => Right,
                Right => Left,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Space {
        Wall,
        Free,
        Slide(Direction),
    }
    use Space::{Free, Slide, Wall};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Offset {
        x: Coordinate,
        y: Coordinate,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct Node {
        x: Coordinate,
        y: Coordinate,
    }

    impl Display for Node {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("(y: {:2},x: {:2})", self.y, self.x))
        }
    }

    impl Add<Offset> for Node {
        type Output = Self;

        fn add(mut self, rhs: Offset) -> Self::Output {
            self.x += rhs.x;
            self.y += rhs.y;
            self
        }
    }
    impl AddAssign<Offset> for Node {
        fn add_assign(&mut self, rhs: Offset) {
            self.x += rhs.x;
            self.y += rhs.y;
        }
    }

    fn trace_path(
        map: &Vec<Vec<Space>>,
        nodes: &Vec<Node>,
        mut node: Node,
        mut direction: Direction,
    ) -> Option<(Node, u16)> {
        if map[node.y as usize][node.x as usize] == Wall {
            return None;
        }

        let mut steps = 1;
        loop {
            if nodes.contains(&node) {
                return Some((node, steps));
            }

            direction = if let Slide(dir) = map[node.y as usize][node.x as usize] {
                if dir.reverse() == direction {
                    return None;
                } else {
                    dir
                }
            } else {
                if direction != Bottom && map[(node.y - 1) as usize][node.x as usize] != Wall {
                    Top
                } else if direction != Top && map[(node.y + 1) as usize][node.x as usize] != Wall {
                    Bottom
                } else if direction != Left && map[node.y as usize][(node.x + 1) as usize] != Wall {
                    Right
                } else {
                    Left
                }
            };

            node += direction.into();
            steps += 1;
        }
    }

    fn add_edge(
        graph: &mut Graph<Node, u16>,
        nodes: &Vec<Node>,
        node_idx: &Vec<NodeIndex>,
        weight: u16,
        start: Node,
        end: Node,
    ) {
        let start_idx = node_idx[nodes.iter().position(|n| *n == start).unwrap()];
        let end_idx = node_idx[nodes.iter().position(|n| *n == end).unwrap()];

        graph.add_edge(start_idx, end_idx, weight);
    }

    let map: Vec<Vec<Space>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Wall,
                    '.' => Free,
                    '^' => Slide(Top),
                    'v' => Slide(Bottom),
                    '<' => Slide(Left),
                    '>' => Slide(Right),
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    fn get_len(graph: &Graph<Node, u16>, path: Vec<NodeIndex>) -> u16 {
        let mut len = 0;
        for i in 0..path.len() - 1 {
            let edge = graph.find_edge(path[i], path[i + 1]).unwrap();
            len += graph[edge];
        }
        len
    }

    let start_node = Node { x: 1, y: 0 };
    let end_node = Node {
        x: (map[0].len() - 2) as Coordinate,
        y: (map.len() - 1) as Coordinate,
    };

    let mut nodes = vec![start_node, end_node];

    for y in 1..map.len() - 1 {
        for x in 1..map[0].len() - 1 {
            if map[y][x] == Wall {
                continue;
            }
            let mut connections = 0u8;

            if map[y - 1][x] != Wall {
                connections += 1;
            }
            if map[y + 1][x] != Wall {
                connections += 1;
            }
            if map[y][x - 1] != Wall {
                connections += 1;
            }
            if map[y][x + 1] != Wall {
                connections += 1;
            }

            assert!(connections != 0);
            if connections != 2 {
                nodes.push(Node {
                    x: x as Coordinate,
                    y: y as Coordinate,
                });
            }
        }
    }

    let mut graph = Graph::new();
    let mut node_idx = Vec::new();

    nodes.iter().copied().for_each(|node| {
        node_idx.push(graph.add_node(node));
    });

    let start_path = trace_path(&map, &nodes, Node { x: 1, y: 1 }, Bottom).unwrap();
    add_edge(
        &mut graph,
        &nodes,
        &node_idx,
        start_path.1,
        Node { x: 1, y: 0 },
        start_path.0,
    );

    for node in nodes.iter().skip(2) {
        let space = map[node.y as usize][node.x as usize];

        if let Slide(dir) = space {
            if let Some((target, spaces)) = trace_path(&map, &nodes, *node + dir.into(), dir) {
                add_edge(&mut graph, &nodes, &node_idx, spaces, *node, target);
            }
        } else {
            for dir in [Top, Bottom, Left, Right] {
                if let Some((target, spaces)) = trace_path(&map, &nodes, *node + dir.into(), dir) {
                    add_edge(&mut graph, &nodes, &node_idx, spaces, *node, target);
                }
            }
        }
    }

    let max = all_simple_paths(&graph, node_idx[0], node_idx[1], 0, None)
        .map(|p: Vec<NodeIndex>| get_len(&graph, p))
        .max()
        .unwrap();

    println!("{max}");
}

fn two(input: String) {
    type Coordinate = i16;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Direction {
        Top,
        Bottom,
        Left,
        Right,
    }
    use Direction::{Bottom, Left, Right, Top};

    impl Into<Offset> for Direction {
        fn into(self) -> Offset {
            match self {
                Top => Offset { x: 0, y: -1 },
                Bottom => Offset { x: 0, y: 1 },
                Left => Offset { x: -1, y: 0 },
                Right => Offset { x: 1, y: 0 },
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Space {
        Wall,
        Free,
    }
    use Space::{Free, Wall};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Offset {
        x: Coordinate,
        y: Coordinate,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct Node {
        x: Coordinate,
        y: Coordinate,
    }

    impl Display for Node {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("(y: {:2},x: {:2})", self.y, self.x))
        }
    }

    impl Add<Offset> for Node {
        type Output = Self;

        fn add(mut self, rhs: Offset) -> Self::Output {
            self.x += rhs.x;
            self.y += rhs.y;
            self
        }
    }
    impl AddAssign<Offset> for Node {
        fn add_assign(&mut self, rhs: Offset) {
            self.x += rhs.x;
            self.y += rhs.y;
        }
    }

    fn trace_path(
        map: &Vec<Vec<Space>>,
        nodes: &Vec<Node>,
        mut node: Node,
        mut direction: Direction,
    ) -> Option<(Node, u16)> {
        if map[node.y as usize][node.x as usize] == Wall {
            return None;
        }

        let mut steps = 1;
        loop {
            if nodes.contains(&node) {
                return Some((node, steps));
            }

            direction =
                if direction != Bottom && map[(node.y - 1) as usize][node.x as usize] != Wall {
                    Top
                } else if direction != Top && map[(node.y + 1) as usize][node.x as usize] != Wall {
                    Bottom
                } else if direction != Left && map[node.y as usize][(node.x + 1) as usize] != Wall {
                    Right
                } else {
                    Left
                };

            node += direction.into();
            steps += 1;
        }
    }

    fn add_edge(
        graph: &mut Graph<Node, u16>,
        nodes: &Vec<Node>,
        node_idx: &Vec<NodeIndex>,
        weight: u16,
        start: Node,
        end: Node,
    ) {
        let start_idx = node_idx[nodes.iter().position(|n| *n == start).unwrap()];
        let end_idx = node_idx[nodes.iter().position(|n| *n == end).unwrap()];

        graph.add_edge(start_idx, end_idx, weight);
    }

    let map: Vec<Vec<Space>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Wall,
                    '.' => Free,
                    '^' => Free,
                    'v' => Free,
                    '<' => Free,
                    '>' => Free,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    fn get_len(graph: &Graph<Node, u16>, path: Vec<NodeIndex>) -> u16 {
        let mut len = 0;
        for i in 0..path.len() - 1 {
            let edge = graph.find_edge(path[i], path[i + 1]).unwrap();
            len += graph[edge];
        }
        len
    }

    let start_node = Node { x: 1, y: 0 };
    let end_node = Node {
        x: (map[0].len() - 2) as Coordinate,
        y: (map.len() - 1) as Coordinate,
    };

    let mut nodes = vec![start_node, end_node];

    for y in 1..map.len() - 1 {
        for x in 1..map[0].len() - 1 {
            if map[y][x] == Wall {
                continue;
            }
            let mut connections = 0u8;

            if map[y - 1][x] != Wall {
                connections += 1;
            }
            if map[y + 1][x] != Wall {
                connections += 1;
            }
            if map[y][x - 1] != Wall {
                connections += 1;
            }
            if map[y][x + 1] != Wall {
                connections += 1;
            }

            assert!(connections != 0);
            if connections != 2 {
                nodes.push(Node {
                    x: x as Coordinate,
                    y: y as Coordinate,
                });
            }
        }
    }

    let mut graph = Graph::new();
    let mut node_idx = Vec::new();

    nodes.iter().copied().for_each(|node| {
        node_idx.push(graph.add_node(node));
    });

    let start_path = trace_path(&map, &nodes, Node { x: 1, y: 1 }, Bottom).unwrap();
    add_edge(
        &mut graph,
        &nodes,
        &node_idx,
        start_path.1,
        Node { x: 1, y: 0 },
        start_path.0,
    );

    for node in nodes.iter().skip(2) {
        for dir in [Top, Bottom, Left, Right] {
            if let Some((target, spaces)) = trace_path(&map, &nodes, *node + dir.into(), dir) {
                add_edge(&mut graph, &nodes, &node_idx, spaces, *node, target);
            }
        }
    }

    let max = all_simple_paths(&graph, node_idx[0], node_idx[1], 0, None)
        .map(|p: Vec<NodeIndex>| get_len(&graph, p))
        .max()
        .unwrap();

    println!("{max}");
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

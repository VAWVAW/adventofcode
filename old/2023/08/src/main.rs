use std::{collections::HashMap, fs};

fn one(input: String) {
    type Node = [char; 3];
    fn str_to_node(s: &str) -> Node {
        let mut chars = s.chars();
        [
            chars.next().unwrap(),
            chars.next().unwrap(),
            chars.next().unwrap(),
        ]
    }

    let mut lines = input.lines();
    let moves_is_right: Vec<bool> = lines.next().unwrap().chars().map(|c| c == 'R').collect();
    lines.next().unwrap();

    let map: HashMap<Node, (Node, Node)> = lines
        .map(|line| {
            let (node, edges) = line.split_once(" = ").unwrap();
            let (left, right) = edges
                .trim_matches(|c| c == '(' || c == ')')
                .split_once(", ")
                .unwrap();
            (str_to_node(node), (str_to_node(left), str_to_node(right)))
        })
        .collect();

    let mut moves = 0;
    let mut current_node = ['A', 'A', 'A'];
    let mut move_index = 0;
    while current_node != ['Z', 'Z', 'Z'] {
        if move_index == moves_is_right.len() {
            move_index = 0;
        }

        let (left, right) = map.get(&current_node).unwrap();
        current_node = if moves_is_right[move_index] {
            *right
        } else {
            *left
        };

        moves += 1;
        move_index += 1;
    }

    println!("{moves}");
}

fn two(input: String) {
    type Node = [char; 3];
    fn str_to_node(s: &str) -> Node {
        let mut chars = s.chars();
        [
            chars.next().unwrap(),
            chars.next().unwrap(),
            chars.next().unwrap(),
        ]
    }
    fn gcd(a: u64, b: u64) -> u64 {
        if a < b {
            gcd(b, a)
        } else if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }
    fn scm(a: u64, b: &u64) -> u64 {
        (a * *b) / gcd(a, *b)
    }

    let mut start_nodes: Vec<Node> = Vec::new();

    let mut lines = input.lines();
    let moves_is_right: Vec<bool> = lines.next().unwrap().chars().map(|c| c == 'R').collect();
    lines.next().unwrap();

    let map: HashMap<Node, (Node, Node)> = lines
        .map(|line| {
            let (node, edges) = line.split_once(" = ").unwrap();
            let (left, right) = edges
                .trim_matches(|c| c == '(' || c == ')')
                .split_once(", ")
                .unwrap();
            let node = str_to_node(node);
            if node[2] == 'A' {
                start_nodes.push(node);
            }
            (node, (str_to_node(left), str_to_node(right)))
        })
        .collect();

    let mut node_loops = Vec::new();

    for start_node in start_nodes {
        let mut move_index = 0;
        let mut moves = 0;
        let mut current_node = start_node;
        loop {
            if current_node[2] == 'Z' {
                break;
            }
            if move_index == moves_is_right.len() {
                move_index = 0;
            }

            let (left, right) = map.get(&current_node).unwrap();
            current_node = if moves_is_right[move_index] {
                *right
            } else {
                *left
            };

            move_index += 1;
            moves += 1;
        }

        node_loops.push(moves);
    }

    println!("{}", node_loops.iter().fold(1, scm));
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
        "t12" => (true, "data_test12.txt"),
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

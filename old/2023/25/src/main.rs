use std::fs;

// Pipe the output of this into `dot -Tsvg` from `graphviz` and find the rightmost node that is
// still in the left group. Then search for the x coordinate of this node.
// Use this coordinate in `grep 'cx=' < output.svg | cut -d' ' -f4 | sed -E 's/c?x?=?"//g' | sort -g | awk '{if ($1 > 28555) sum1 += 1; else sum2 += 1}; END {print sum1 * sum2}'`.
fn one(input: String) {
    print!("graph {{ ");
    for line in input.lines() {
        let (start, targets) = line.split_once(": ").unwrap();
        for target in targets.split(' ') {
            print!("{start} -- {target}; ");
        }
    }
    println!("}}");
}

fn two(_input: String) {}

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

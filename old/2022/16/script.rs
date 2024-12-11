use std::fs;
use std::rc::{Rc, Weak};
use std::collections::HashMap;
use std::cell::RefCell;


#[derive(Debug)]
struct Valve {
    name: String,
    flow: i32,
    others: RefCell<Vec<Weak<Valve>>>,
    others_s: Vec<String>
}

#[derive(Debug)]
struct State {
    current: Rc<Valve>,
    valves: Rc<HashMap<String, Rc<Valve>>>,
    score: i32,
    open_valves: Rc<Vec<String>>,
    time: i32,
    action: String
}

impl State {
    fn get_score(&self) -> i32 {
        if self.time > 15 {
            println!("{}", self.time);
        }
        if self.time == 0 {
            return self.score
        }
        let mut scores = Vec::new();
        // open current valve if possible
        if !self.valves.contains_key(&self.current.name) {
            let mut open_valves = self.open_valves.as_ref().clone();
            open_valves.push(self.current.name.clone());
            scores.push(State{
                open_valves: Rc::new(open_valves),
                current: Rc::clone(&self.current),
                valves: Rc::clone(&self.valves),
                time: self.time - 1,
                score: self.score + (self.time - 1) * self.current.flow,
                action: format!("open {}", self.current.name),
            }.get_score())
        }
        // move to other valve
        for valve_weak in self.current.others.borrow().iter() {
            let valve = valve_weak.upgrade().unwrap();
            scores.push(State{
                valves: Rc::clone(&self.valves),
                time: self.time -1,
                score: self.score,
                action: format!("move to {}", valve.name),
                open_valves: Rc::clone(&self.open_valves),
                current: valve,
            }.get_score())
        }
        return *scores.iter().min().unwrap()
    }
}

fn one(){
    // read valves
    let content = fs::read_to_string("data_test.txt").expect("file not read");
    let mut valves: HashMap<String, Rc<Valve>> = HashMap::new();
    let mut open_valves: Vec<String> = Vec::new();
    for line in content.lines() {
        let values_s = line
            .replace("Valve ", "")
            .replace(" has flow rate=", ", ")
            .replace("; tunnels lead to valves ", ", ")
            .replace("; tunnel leads to valve ", ", ");
        let values = values_s.split(", ").collect::<Vec<_>>();
        let name = values[0];
        let flow = values[1].parse::<i32>().unwrap();
        let mut others_s = Vec::new();
        for other in &values[2..] {
            others_s.push(other.to_string())
        }

        valves.insert(
            name.to_string(),
            Rc::new(Valve{
                flow,
                others_s,
                name: name.to_string(),
                others: RefCell::new(Vec::new()),
            })
        );
        if flow == 0 {
            open_valves.push(name.to_string());
        }
    }

    for (_name, valve) in &valves {
        let mut others = Vec::new();
        for other in &valve.others_s {
            others.push(Rc::downgrade(&valves[other]));
        }
        valve.others.replace(others);
    }

    //generate states
    let root = State{
        current: Rc::clone(&valves["AA"]),
        valves: Rc::new(valves),
        score: 0,
        open_valves: Rc::new(open_valves),
        time: 30,
        action: "start".to_string(),
    };
    println!("{}", root.get_score());
}

fn main(){
    one();
}
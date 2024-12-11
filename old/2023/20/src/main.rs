use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
use std::fs;
use std::ops::Not;

fn one(input: String) {
    const INPUT_SIGNAL: Signal = Signal {
        t: Low,
        target: "broadcaster",
        sender: "button",
    };

    #[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
    enum SignalType {
        High,
        #[default]
        Low,
    }
    use SignalType::{High, Low};

    impl Not for SignalType {
        type Output = Self;

        fn not(self) -> Self::Output {
            match self {
                High => Low,
                Low => High,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct Signal<'a> {
        t: SignalType,
        target: &'a str,
        sender: &'a str,
    }

    impl Display for Signal<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!(
                "{} -{}-> {}",
                self.sender,
                match self.t {
                    High => "high",
                    Low => "low",
                },
                self.target,
            ))
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    enum Module<'a> {
        Broadcast {
            targets: Vec<&'a str>,
        },
        FlipFlop {
            targets: Vec<&'a str>,
            state: SignalType,
        },
        Conjection {
            targets: Vec<&'a str>,
            inputs: HashMap<&'a str, SignalType>,
        },
        Nop,
    }
    use Module::{Broadcast, Conjection, FlipFlop, Nop};

    impl<'a> Module<'a> {
        fn handle_signal(&mut self, signal: Signal<'a>) -> VecDeque<Signal<'a>> {
            match self {
                Broadcast { targets } => targets
                    .iter()
                    .map(|target| Signal {
                        target,
                        sender: signal.target,
                        t: signal.t,
                    })
                    .collect(),
                FlipFlop { targets, state } => {
                    if High == signal.t {
                        VecDeque::new()
                    } else {
                        *state = !*state;
                        targets
                            .iter()
                            .map(|target| Signal {
                                target,
                                t: *state,
                                sender: signal.target,
                            })
                            .collect()
                    }
                }
                Conjection { targets, inputs } => {
                    inputs.insert(signal.sender, signal.t);

                    let new_signal = if inputs.iter().all(|(_, t)| High == *t) {
                        Low
                    } else {
                        High
                    };
                    targets
                        .iter()
                        .map(|target| Signal {
                            target,
                            t: new_signal,
                            sender: signal.target,
                        })
                        .collect()
                }
                Nop => Default::default(),
            }
        }
        /// must be called with each module connected to this
        fn init(&mut self, sender: &'a str) {
            if let Conjection { targets: _, inputs } = self {
                inputs.insert(sender, Low);
            }
        }
        fn get_targets(&self) -> &[&'a str] {
            match self {
                Broadcast { targets } => targets,
                FlipFlop { targets, state: _ } => targets,
                Conjection { targets, inputs: _ } => targets,
                Nop => Default::default(),
            }
        }
    }

    let mut modules: HashMap<&str, Module> = input
        .lines()
        .map(|line| {
            let (id, targets_s) = line.split_once(" -> ").unwrap();
            let targets: Vec<_> = targets_s.split(", ").collect();

            let (module_type, name) = id.split_at(1);

            match module_type {
                "b" => (id, Broadcast { targets }),
                "%" => (
                    name,
                    FlipFlop {
                        targets,
                        state: Low,
                    },
                ),
                "&" => (
                    name,
                    Conjection {
                        targets,
                        inputs: HashMap::new(),
                    },
                ),
                _ => unreachable!(),
            }
        })
        .collect();

    // init modules
    {
        let keys: Vec<_> = modules.keys().copied().collect();
        for name in keys {
            let targets = Vec::from(modules.get(name).unwrap().get_targets());

            for target in targets {
                if let Some(module) = modules.get_mut(target) {
                    module.init(name);
                } else {
                    modules.insert(target, Nop);
                }
            }
        }
    }

    let mut n_low: u32 = 0;
    let mut n_high: u32 = 0;

    for _ in 0..1000 {
        let mut signals = VecDeque::from([INPUT_SIGNAL]);
        while let Some(signal) = signals.pop_front() {
            match signal.t {
                High => n_high += 1,
                Low => n_low += 1,
            }

            let target = signal.target;
            let mut new_signals = modules.get_mut(target).unwrap().handle_signal(signal);
            signals.append(&mut new_signals);
        }
    }

    println!("{}", n_low * n_high);
}

fn two(input: String) {
    const INPUT_SIGNAL: Signal = Signal {
        t: Low,
        target: "broadcaster",
        sender: "button",
    };

    #[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
    enum SignalType {
        High,
        #[default]
        Low,
    }
    use SignalType::{High, Low};

    impl Not for SignalType {
        type Output = Self;

        fn not(self) -> Self::Output {
            match self {
                High => Low,
                Low => High,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct Signal<'a> {
        t: SignalType,
        target: &'a str,
        sender: &'a str,
    }

    impl Display for Signal<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!(
                "{} -{}-> {}",
                self.sender,
                match self.t {
                    High => "high",
                    Low => "low",
                },
                self.target,
            ))
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    enum Module<'a> {
        Broadcast {
            targets: Vec<&'a str>,
        },
        FlipFlop {
            targets: Vec<&'a str>,
            state: SignalType,
        },
        Conjection {
            targets: Vec<&'a str>,
            inputs: HashMap<&'a str, SignalType>,
        },
        Nop,
    }
    use Module::{Broadcast, Conjection, FlipFlop, Nop};

    impl<'a> Module<'a> {
        fn handle_signal(&mut self, signal: Signal<'a>) -> VecDeque<Signal<'a>> {
            match self {
                Broadcast { targets } => targets
                    .iter()
                    .map(|target| Signal {
                        target,
                        sender: signal.target,
                        t: signal.t,
                    })
                    .collect(),
                FlipFlop { targets, state } => {
                    if High == signal.t {
                        VecDeque::new()
                    } else {
                        *state = !*state;
                        targets
                            .iter()
                            .map(|target| Signal {
                                target,
                                t: *state,
                                sender: signal.target,
                            })
                            .collect()
                    }
                }
                Conjection { targets, inputs } => {
                    inputs.insert(signal.sender, signal.t);

                    let new_signal = if inputs.iter().all(|(_, t)| High == *t) {
                        Low
                    } else {
                        High
                    };
                    targets
                        .iter()
                        .map(|target| Signal {
                            target,
                            t: new_signal,
                            sender: signal.target,
                        })
                        .collect()
                }
                Nop => Default::default(),
            }
        }
        /// must be called with each module connected to this
        fn init(&mut self, sender: &'a str) {
            if let Conjection { targets: _, inputs } = self {
                inputs.insert(sender, Low);
            }
        }
        fn get_targets(&self) -> &[&'a str] {
            match self {
                Broadcast { targets } => targets,
                FlipFlop { targets, state: _ } => targets,
                Conjection { targets, inputs: _ } => targets,
                Nop => Default::default(),
            }
        }
    }

    let mut modules: HashMap<&str, Module> = input
        .lines()
        .map(|line| {
            let (id, targets_s) = line.split_once(" -> ").unwrap();
            let targets: Vec<_> = targets_s.split(", ").collect();

            let (module_type, name) = id.split_at(1);

            match module_type {
                "b" => (id, Broadcast { targets }),
                "%" => (
                    name,
                    FlipFlop {
                        targets,
                        state: Low,
                    },
                ),
                "&" => (
                    name,
                    Conjection {
                        targets,
                        inputs: HashMap::new(),
                    },
                ),
                _ => unreachable!(),
            }
        })
        .collect();

    // init modules
    {
        let keys: Vec<_> = modules.keys().copied().collect();
        for name in keys {
            let targets = Vec::from(modules.get(name).unwrap().get_targets());

            for target in targets {
                if let Some(module) = modules.get_mut(target) {
                    module.init(name);
                } else {
                    modules.insert(target, Nop);
                }
            }
        }
    }

    // find the modules that need to be high to make the output high.
    let important_modules = ["tf", "vq", "db", "ln"];


    // find the lcm of the numbers printed for each module

    // tf: 3923
    // vq: 4007
    // db: 3929
    // ln: 4091


    let mut current_press = 1;

    let press = 'outer: loop {
        let mut signals = VecDeque::from([INPUT_SIGNAL]);
        while let Some(signal) = signals.pop_front() {
            if important_modules.contains(&signal.sender) && signal.t == High {
                println!("{}: {current_press}", signal.sender);
            }
            if signal.target == "rx" && signal.t == Low {
                break 'outer current_press;
            }

            let target = signal.target;
            let mut new_signals = modules.get_mut(target).unwrap().handle_signal(signal);
            signals.append(&mut new_signals);
        }

        current_press += 1;
    };

    println!("{press}");
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
        _ => (true, "data_test1.txt"),
    };
    let input = fs::read_to_string(file_name).unwrap();

    if execute_first {
        one(input);
    } else {
        two(input);
    }
}

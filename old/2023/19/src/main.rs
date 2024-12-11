use std::collections::HashMap;
use std::fs;

fn one(input: String) {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Match {
        Gt(u32),
        Lt(u32),
    }
    use Match::{Gt, Lt};

    impl Match {
        fn matches(&self, v: u32) -> bool {
            match self {
                Gt(t) => v > *t,
                Lt(t) => v < *t,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Rule {
        A(Match),
        M(Match),
        S(Match),
        X(Match),
        Wildcard,
    }
    use Rule::{Wildcard, A, M, S, X};

    impl Rule {
        fn matches(&self, part: &Part) -> bool {
            match self {
                A(m) => m.matches(part.a),
                M(m) => m.matches(part.m),
                S(m) => m.matches(part.s),
                X(m) => m.matches(part.x),
                Wildcard => true,
            }
        }
    }

    impl From<&str> for Rule {
        /// This panics if used incorrectly.
        fn from(value: &str) -> Self {
            let n: u32 = value.get(2..).unwrap().parse().unwrap();
            let m = match value.get(1..=1) {
                Some("<") => Lt(n),
                Some(">") => Gt(n),
                _ => unreachable!(),
            };
            match value.get(0..=0) {
                Some("a") => A(m),
                Some("m") => M(m),
                Some("s") => S(m),
                Some("x") => X(m),
                _ => unreachable!(),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct Part {
        a: u32,
        m: u32,
        s: u32,
        x: u32,
    }

    let mut workflows: HashMap<&str, Vec<(&str, Rule)>> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();

    let mut lines = input.lines();

    // fill workflows
    for line in &mut lines {
        if line == "" {
            break;
        }

        let (name, rules_s) = line.trim_end_matches('}').split_once('{').unwrap();

        let rules: Vec<(&str, Rule)> = rules_s
            .split(',')
            .map(|rule_s| {
                if let Some((match_s, name)) = rule_s.split_once(':') {
                    (name, match_s.into())
                } else {
                    (rule_s, Wildcard)
                }
            })
            .collect();
        workflows.insert(name, rules);
    }

    // fill parts
    for line in lines {
        let mut elems = line
            .trim_end_matches('}')
            .split(',')
            .map(|e| e.split_once('=').unwrap().1.parse().unwrap());
        parts.push(Part {
            x: elems.next().unwrap(),
            m: elems.next().unwrap(),
            a: elems.next().unwrap(),
            s: elems.next().unwrap(),
        })
    }

    let mut accepted = Vec::new();

    for part in parts {
        let mut workflow_name = "in";

        loop {
            if workflow_name == "A" {
                accepted.push(part);
                break;
            }
            if workflow_name == "R" {
                break;
            }
            let workflow = workflows.get(workflow_name).unwrap();
            for (target, rule) in workflow {
                if rule.matches(&part) {
                    workflow_name = target;
                    break;
                }
            }
        }
    }

    let sum: u32 = accepted
        .iter()
        .map(|part| part.a + part.m + part.s + part.x)
        .sum();

    println!("{sum}");
}

fn two(input: String) {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Match {
        Gt(u16),
        Lt(u16),
    }
    use Match::{Gt, Lt};

    impl Match {
        fn matches(&self, v: Range) -> (Range, Range) {
            match self {
                Gt(t) => (
                    Range {
                        start: t + 1,
                        end: v.end,
                    },
                    Range {
                        start: v.start,
                        end: *t,
                    },
                ),
                Lt(t) => (
                    Range {
                        start: v.start,
                        end: t - 1,
                    },
                    Range {
                        start: *t,
                        end: v.end,
                    },
                ),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Rule {
        A(Match),
        M(Match),
        S(Match),
        X(Match),
        Wildcard,
    }
    use Rule::{Wildcard, A, M, S, X};

    impl Rule {
        fn matches(&self, part: PartRange) -> (PartRange, PartRange) {
            match self {
                A(m) => {
                    let (t, f) = m.matches(part.a);
                    (PartRange { a: t, ..part }, PartRange { a: f, ..part })
                }
                M(m) => {
                    let (t, f) = m.matches(part.m);
                    (PartRange { m: t, ..part }, PartRange { m: f, ..part })
                }
                S(m) => {
                    let (t, f) = m.matches(part.s);
                    (PartRange { s: t, ..part }, PartRange { s: f, ..part })
                }
                X(m) => {
                    let (t, f) = m.matches(part.x);
                    (PartRange { x: t, ..part }, PartRange { x: f, ..part })
                }
                Wildcard => (
                    part,
                    // empty range
                    PartRange {
                        a: Range { start: 42, end: 0 },
                        ..part
                    },
                ),
            }
        }
    }

    impl From<&str> for Rule {
        /// This panics if used incorrectly.
        fn from(value: &str) -> Self {
            let n: u16 = value.get(2..).unwrap().parse().unwrap();
            let m = match value.get(1..=1) {
                Some("<") => Lt(n),
                Some(">") => Gt(n),
                _ => unreachable!(),
            };
            match value.get(0..=0) {
                Some("a") => A(m),
                Some("m") => M(m),
                Some("s") => S(m),
                Some("x") => X(m),
                _ => unreachable!(),
            }
        }
    }

    /// both values are inclusive
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct Range {
        start: u16,
        end: u16,
    }
    impl Range {
        fn has_elements(&self) -> bool {
            self.start <= self.end
        }
        fn element_n(&self) -> u64 {
            (self.end - self.start + 1) as u64
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct PartRange {
        a: Range,
        m: Range,
        s: Range,
        x: Range,
    }

    impl PartRange {
        fn has_elements(&self) -> bool {
            self.a.has_elements()
                && self.m.has_elements()
                && self.s.has_elements()
                && self.x.has_elements()
        }
    }

    type Workflows<'a> = HashMap<&'a str, Vec<(&'a str, Rule)>>;

    fn check_parts(
        mut parts: PartRange,
        workflow_name: &str,
        workflows: &Workflows,
        accepted: &mut Vec<PartRange>,
    ) {
        if workflow_name == "A" {
            accepted.push(parts);
            return;
        }
        if workflow_name == "R" {
            return;
        }
        if !parts.has_elements() {
            return;
        }

        let workflow = workflows.get(workflow_name).unwrap();
        for (name, rule) in workflow {
            let (m_parts, o_parts) = rule.matches(parts);

            check_parts(m_parts, name, workflows, accepted);

            if !o_parts.has_elements() {
                break;
            }
            parts = o_parts;
        }
    }

    let workflows: Workflows = input
        .lines()
        .take_while(|line| line != &"")
        .map(|line| {
            let (name, rules_s) = line.trim_end_matches('}').split_once('{').unwrap();

            let rules: Vec<(&str, Rule)> = rules_s
                .split(',')
                .map(|rule_s| {
                    if let Some((match_s, name)) = rule_s.split_once(':') {
                        (name, match_s.into())
                    } else {
                        (rule_s, Wildcard)
                    }
                })
                .collect();
            (name, rules)
        })
        .collect();

    let mut accepted = Vec::new();

    check_parts(
        PartRange {
            a: Range { start: 1, end: 4000 },
            m: Range { start: 1, end: 4000 },
            s: Range { start: 1, end: 4000 },
            x: Range { start: 1, end: 4000 },
        },
        "in",
        &workflows,
        &mut accepted,
    );

    let combinations: u64 = accepted.into_iter().map(|parts| {
        parts.a.element_n() * parts.m.element_n() * parts.s.element_n() * parts.x.element_n()
    }).sum();

    println!("{combinations}");
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

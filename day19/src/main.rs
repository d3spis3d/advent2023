use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

trait Rule {
    fn apply(&self, part: &Part) -> bool;
}

struct GreaterThanRule(String, u64);

impl Rule for GreaterThanRule {
    fn apply(&self, part: &Part) -> bool {
        part.get(&self.0) > self.1
    }
}

impl GreaterThanRule {
    fn from(str: String) -> Self {
        let p = str.split(">").collect::<Vec<&str>>();
        let field = p[0].to_owned();
        let v = p[1].parse::<u64>().unwrap();
        GreaterThanRule(field, v)
    }
}

struct LessThanRule(String, u64);

impl Rule for LessThanRule {
    fn apply(&self, part: &Part) -> bool {
        part.get(&self.0) < self.1
    }
}

impl LessThanRule {
    fn from(str: String) -> Self {
        let p = str.split("<").collect::<Vec<&str>>();
        let field = p[0].to_owned();
        let v = p[1].parse::<u64>().unwrap();
        LessThanRule(field, v)
    }
}

struct NoOpRule;

impl Rule for NoOpRule {
    fn apply(&self, _: &Part) -> bool {
        true
    }
}

#[derive(Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn from(str: String) -> Self {
        let trimmed = str.trim_start_matches("{").trim_end_matches("}");
        let parts = trimmed.split(",").collect::<Vec<&str>>();

        let x = parts[0].trim_start_matches("x=").parse::<u64>().unwrap();
        let m = parts[1].trim_start_matches("m=").parse::<u64>().unwrap();
        let a = parts[2].trim_start_matches("a=").parse::<u64>().unwrap();
        let s = parts[3].trim_start_matches("s=").parse::<u64>().unwrap();

        Part { x, m, a, s }
    }

    fn get(&self, field: &str) -> u64 {
        match field {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => panic!("no field"),
        }
    }

    fn sum(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

struct Workflow {
    name: String,
    rules: Vec<(Box<dyn Rule>, String)>,
}

impl Workflow {
    fn from(str: String) -> Self {
        let mut parts = str.split("{");
        let name = parts.next().unwrap();

        let remaining = parts.next().unwrap();
        let body = remaining.replace("}", "");
        let rules = body
            .split(",")
            .into_iter()
            .map(|r| {
                let rule: (Box<dyn Rule>, String) = if r.contains("<") {
                    let p = r.split(":").collect::<Vec<&str>>();
                    (
                        Box::new(LessThanRule::from(p[0].to_owned())),
                        p[1].to_owned(),
                    )
                } else if r.contains(">") {
                    let p = r.split(":").collect::<Vec<&str>>();
                    (
                        Box::new(GreaterThanRule::from(p[0].to_owned())),
                        p[1].to_owned(),
                    )
                } else {
                    (Box::new(NoOpRule {}), r.to_owned())
                };
                rule
            })
            .collect::<Vec<(Box<dyn Rule>, String)>>();

        Workflow {
            name: name.to_owned(),
            rules,
        }
    }
}

fn apply_workflow(p: &Part, w: &Workflow) -> String {
    let matching_rule = w.rules.iter().find(|r| r.0.apply(p)).unwrap();
    matching_rule.1.clone()
}

fn main() {
    let Ok(mut lines) = read_lines("./input.txt") else {
        panic!("couldn't read input");
    };

    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();

    let mut accepted: Vec<&Part> = Vec::new();

    loop {
        let line = lines.next().unwrap().unwrap();
        if line == "" {
            break;
        }

        let w = Workflow::from(line);
        workflows.insert(w.name.to_owned(), w);
    }

    for l in lines {
        let line = l.unwrap();

        let part = Part::from(line);
        parts.push(part);
    }

    for part in parts.iter() {
        let mut w = workflows.get("in").unwrap();

        loop {
            let result = apply_workflow(part, w);
            if result == "R" {
                break;
            }
            if result == "A" {
                accepted.push(part);
                break;
            }

            w = workflows.get(&result).unwrap();
        }
    }

    let result: u64 = accepted.iter().map(|a| a.sum()).sum();
    println!("result: {}", result);
}

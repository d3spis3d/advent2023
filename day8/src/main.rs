use reformation::Reformation;
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

#[derive(Reformation, Debug)]
#[reformation(r"{name} = \({left}, {right}\)")]
struct Node {
    name: String,
    left: String,
    right: String,
}

fn follow(connections: HashMap<String, (String, String)>, directions: String) -> u64 {
    let mut steps = 0;
    let mut current = connections.get("AAA").unwrap();

    // println!("{:?}", current);

    loop {
        for c in directions.chars() {
            steps = steps + 1;

            match c {
                'L' => {
                    let next = &current.0;
                    if next == "ZZZ" {
                        return steps;
                    }
                    current = connections.get(next).unwrap();
                }
                'R' => {
                    let next = &current.1;
                    if next == "ZZZ" {
                        return steps;
                    }
                    current = connections.get(next).unwrap();
                }
                _ => panic!("arggggggh"),
            };
            // println!("{:?}", current);
        }
    }
}

fn main() {
    let Ok(mut lines) = read_lines("./input.txt") else {
        panic!("couldn't read input");
    };

    let directions = lines.next().unwrap().unwrap();
    lines.next(); // blank

    let mut connections: HashMap<String, (String, String)> = HashMap::new();

    for l in lines {
        let line = l.unwrap();
        let node = Node::parse(&line).unwrap();

        connections.insert(node.name, (node.left, node.right));
    }

    let steps = follow(connections, directions);

    println!("{}", steps);
}

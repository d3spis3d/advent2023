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

fn at_end(c: &char, current: &&(String, String)) -> bool {
    let next = match c {
        'L' => &current.0,
        'R' => &current.1,
        _ => panic!("nooooo!"),
    };

    if next.ends_with("Z") {
        return true;
    }

    false
}

fn get_next<'a>(
    c: char,
    current: &(String, String),
    connections: &'a HashMap<&String, (String, String)>,
) -> &'a (String, String) {
    match c {
        'L' => {
            let next = &current.0;
            return connections.get(next).unwrap();
        }
        'R' => {
            let next = &current.1;
            return connections.get(next).unwrap();
        }
        _ => panic!("arggggggh"),
    };
}

fn follow(
    connections: &HashMap<&String, (String, String)>,
    directions: &String,
    starting_node: &String,
) -> u64 {
    let mut steps = 0;

    let mut current: &(String, String) = connections.get(starting_node).unwrap();

    // println!("{:?}", current);

    loop {
        for c in directions.chars() {
            steps = steps + 1;
            // println!("{}", c);

            if at_end(&c, &current) {
                return steps;
            }

            current = get_next(c, &current, &connections);
            // println!("{:?}", current);
        }
    }
}

fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn main() {
    let Ok(mut lines) = read_lines("./input.txt") else {
        panic!("couldn't read input");
    };

    let directions = lines.next().unwrap().unwrap();
    lines.next(); // blank

    let mut connections: HashMap<&String, (String, String)> = HashMap::new();

    let mut starting_nodes: Vec<String> = Vec::new();
    let mut all_nodes: Vec<Node> = Vec::new();

    for l in lines {
        let line = l.unwrap();
        let node = Node::parse(&line).unwrap();

        if node.name.ends_with("A") {
            starting_nodes.push(node.name.clone());
        }

        all_nodes.push(node);
    }

    all_nodes.iter().for_each(|n| {
        connections.insert(&n.name, (n.left.clone(), n.right.clone()));
    });

    // println!("{:?}", starting_nodes);
    let cycles: Vec<u64> = starting_nodes
        .iter()
        .map(|sn| follow(&connections, &directions, sn))
        .collect::<Vec<u64>>();
    let steps = lcm(&cycles);
    println!("{}", steps);
}

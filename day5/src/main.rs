use regex::Regex;
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

fn main() {
    let Ok(mut lines) = read_lines("./test.txt") else {
        panic!("couldn't read input");
    };

    let seed_line = lines.next().unwrap().unwrap();
    let seeds = seed_line
        .trim_start_matches("seeds: ")
        .split(" ")
        .collect::<Vec<&str>>();

    let mut blank = lines.next().unwrap().unwrap(); // skip blank
    if blank != "" {
        println!("{}", blank);
        panic!("argggg not blank");
    }

    let seed_to_soil_header = lines.next().unwrap().unwrap();
    if seed_to_soil_header != "seed-to-soil map:" {
        panic!("bad header");
    }

    let mut seed_to_soil: HashMap<u32, u32> = HashMap::new();

    let re = Regex::new(r"^(?<original>[0-9]+) (?<new>[0-9]+) (?<range>[0-9]+)$").unwrap();

    loop {
        let l = lines.next().unwrap().unwrap();
        if l == "" {
            break;
        }

        let caps = re.captures(&l).unwrap();
        let seed = caps
            .name("original")
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();
        let soil = caps.name("new").unwrap().as_str().parse::<u32>().unwrap();
        let range = caps.name("range").unwrap().as_str().parse::<u32>().unwrap();

        for i in 0..range {
            seed_to_soil.insert(seed + i, soil + i);
        }
    }

    // make that into a function to call for each section
}

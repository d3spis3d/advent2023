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
}

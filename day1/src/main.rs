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
    let Ok(lines) = read_lines("./input.txt") else {
        panic!("couldn't read input");
    };

    let mut nums: Vec<u32> = Vec::new();

    for l in lines {
        let line = l.unwrap();

        let first = line.chars().find(|c| {
            if let Some(_) = c.to_digit(10) {
                true
            } else {
                false
            }
        });

        let last = line.chars().rev().find(|c| {
            if let Some(_) = c.to_digit(10) {
                true
            } else {
                false
            }
        });

        let s = format!("{}{}", first.unwrap(), last.unwrap());
        let n = s.parse::<u32>().unwrap();
        nums.push(n);
    }

    let result: u32 = nums.iter().sum();
    println!("{}", result);
}

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

fn calculate_times(time: u64, distance: u64) -> u64 {
    let mut winning_times = 0;

    for t in 0..time + 1 {
        if t * (time - t) > distance {
            winning_times = winning_times + 1;
        }
    }

    winning_times
}

fn main() {
    let Ok(mut lines) = read_lines("./input.txt") else {
        panic!("couldn't read input");
    };

    let times = lines.next().unwrap().unwrap();
    let distances = lines.next().unwrap().unwrap();

    let t = times
        .trim_start_matches("Time:")
        .trim()
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();

    let d = distances
        .trim_start_matches("Distance:")
        .trim()
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();

    let result = calculate_times(t, d);

    println!("{}", result);
}

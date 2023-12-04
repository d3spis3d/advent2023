use reformation::Reformation;
use std::collections::HashSet;
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

    let all_lines: Vec<String> = lines.map(|l| l.unwrap()).collect::<Vec<String>>();
    let line_count = all_lines.len();

    let mut points: Vec<u32> = vec![1; line_count];
    let mut current_card: usize = 0;

    for l in all_lines.into_iter() {
        let line = l;

        let mut parts = line.split(": ").skip(1).next().unwrap().split("|");
        let winning = parts.next().unwrap();
        let numbers = parts.next().unwrap();

        // println!("{:?}", winning);
        let winning = winning.split(" ").collect::<HashSet<&str>>();

        let number_of_winners = numbers
            .split(" ")
            .filter(|n| *n != " " && *n != "")
            .filter(|n| winning.contains(n))
            .count();

        let multiplier = points[current_card];

        for i in current_card + 1..current_card + number_of_winners + 1 {
            if i >= line_count {
                break;
            }

            points[i] = points[i] + multiplier;
        }

        println!("{:?}", points);
        current_card = current_card + 1;
    }

    println!("{:?}", points);

    let result: u32 = points.iter().sum::<u32>();
    println!("result: {}", result);
}

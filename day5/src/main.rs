use rangemap::RangeMap;
use regex::Regex;
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

fn create_map(
    lines: &mut io::Lines<io::BufReader<File>>,
    header: &str,
) -> RangeMap<u64, (u64, u64, u64)> {
    let mut map: RangeMap<u64, (u64, u64, u64)> = RangeMap::new();

    let re = Regex::new(r"^(?<new>[0-9]+) (?<original>[0-9]+) (?<range>[0-9]+)$").unwrap();

    let header_line = lines.next().unwrap().unwrap();
    if header_line != header {
        panic!("bad header");
    }

    loop {
        let l = lines.next();
        match l {
            None => break,
            _ => {}
        }

        let line = l.unwrap().unwrap();

        if line == "" {
            break;
        }

        let caps = re.captures(&line).unwrap();
        let original = caps
            .name("original")
            .unwrap()
            .as_str()
            .parse::<u64>()
            .unwrap();
        let new = caps.name("new").unwrap().as_str().parse::<u64>().unwrap();
        let range = caps.name("range").unwrap().as_str().parse::<u64>().unwrap();

        // println!("{} {} {}", original, new, range);
        map.insert(original..original + range, (original, new, range));
    }

    return map;
}

fn main() {
    let Ok(mut lines) = read_lines("./input.txt") else {
        panic!("couldn't read input");
    };

    let seed_line = lines.next().unwrap().unwrap();
    let seeds = seed_line
        .trim_start_matches("seeds: ")
        .split(" ")
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|seed_range| {
            let mut seeds = Vec::new();
            let start = seed_range[0].parse::<u64>().unwrap();
            let range = seed_range[1].parse::<u64>().unwrap();

            for i in start..start + range {
                seeds.push(i);
            }

            seeds
        })
        .flatten()
        .collect::<Vec<u64>>();

    let blank = lines.next().unwrap().unwrap(); // skip blank
    if blank != "" {
        println!("{}", blank);
        panic!("argggg not blank");
    }

    let headers = vec![
        "seed-to-soil map:",
        "soil-to-fertilizer map:",
        "fertilizer-to-water map:",
        "water-to-light map:",
        "light-to-temperature map:",
        "temperature-to-humidity map:",
        "humidity-to-location map:",
    ];

    let maps = headers
        .iter()
        .map(|h| create_map(&mut lines, h))
        .collect::<Vec<RangeMap<u64, (u64, u64, u64)>>>();

    let result = seeds
        .iter()
        .map(|s| {
            // let x = s.parse::<u64>().unwrap();
            maps.iter().fold(*s, |acc, map| {
                let new = map.get(&acc);
                // println!("{:?} {:?}", acc, new);
                match new {
                    Some((original, new, _)) => new + (acc - original),
                    None => acc,
                }
                // println!("from {} to {}", acc, new);
            })
        })
        .min();

    println!("result: {}", result.unwrap());
}

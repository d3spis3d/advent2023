use std::collections::{HashMap, VecDeque};
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

fn match_digit(chars: &Vec<char>) -> Option<(isize, i32)> {
    let c = chars.iter();

    let result = c.enumerate().find(|(_, c)| {
        if let Some(_) = c.to_digit(10) {
            true
        } else {
            false
        }
    });

    match result {
        Some((i, c)) => {
            let x = c.to_digit(10).unwrap();
            Some((i as isize, x as i32))
        }
        None => None,
    }
}

fn match_size(
    chars: &Vec<char>,
    size: u8,
    words: &HashMap<String, i32>,
    reverse: bool,
) -> Option<(isize, i32)> {
    let mut dq: VecDeque<&char> = VecDeque::new();

    let mut c = chars.iter();

    for _ in 0..size {
        if let Some(k) = c.next() {
            if reverse {
                dq.push_front(k);
            } else {
                dq.push_back(k);
            }
        } else {
            return None;
        }
    }

    let mut index: isize = 0;

    let s = dq.iter().map(|c| **c).collect::<String>();
    if let Some(x) = words.get(&s) {
        return Some((index, *x));
    };

    while let Some(x) = c.next() {
        index = index + 1;
        if reverse {
            dq.pop_back();
            dq.push_front(x);
        } else {
            dq.pop_front();
            dq.push_back(x);
        }

        let s = dq.iter().map(|c| **c).collect::<String>();
        if let Some(x) = words.get(&s) {
            return Some((index, *x));
        };
    }

    None
}

fn find_first_match(matches: &Vec<Option<(isize, i32)>>) -> i32 {
    // println!("{:?}", matches);
    let mut filtered: Vec<(isize, i32)> = matches
        .iter()
        .filter(|f| match f {
            None => false,
            _ => true,
        })
        .map(|f| f.unwrap())
        .filter(|f| {
            let (i, _) = *f;
            if i == -1 {
                false
            } else {
                true
            }
        })
        .collect();

    filtered.sort_by(|a, b| {
        let (i, _) = a;
        let (k, _) = b;
        i.cmp(k)
    });

    let (_, result) = filtered[0];
    result
}

fn match_number(chars: Vec<char>) -> u32 {
    let three = &HashMap::from([
        (String::from("one"), 1),
        (String::from("two"), 2),
        (String::from("six"), 6),
    ]);
    let four = &HashMap::from([
        (String::from("four"), 4),
        (String::from("five"), 5),
        (String::from("nine"), 9),
    ]);
    let five = &HashMap::from([
        (String::from("three"), 3),
        (String::from("seven"), 7),
        (String::from("eight"), 8),
    ]);

    let forward = Vec::from([
        match_digit(&chars),
        match_size(&chars, 3, three, false),
        match_size(&chars, 4, four, false),
        match_size(&chars, 5, five, false),
    ]);

    let forward_match = find_first_match(&forward);

    let mut rev_chars = chars.clone();
    rev_chars.reverse();

    let backward = Vec::from([
        match_digit(&rev_chars),
        match_size(&rev_chars, 3, three, true),
        match_size(&rev_chars, 4, four, true),
        match_size(&rev_chars, 5, five, true),
    ]);

    let backward_match = find_first_match(&backward);

    let s = format!("{}{}", forward_match, backward_match);
    // println!("{}", s);
    s.parse::<u32>().unwrap()
}

fn main() {
    let Ok(lines) = read_lines("./input.txt") else {
        panic!("couldn't read input");
    };

    let mut nums: Vec<u32> = Vec::new();

    for l in lines {
        let line = l.unwrap();

        let forward = line.chars().collect::<Vec<char>>();

        let result = match_number(forward);

        nums.push(result);
    }

    let result: u32 = nums.iter().sum();
    println!("{}", result);
}

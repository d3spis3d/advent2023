use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct PartNumber {
    number: u32,
    start: (u32, u32),
    end: (u32, u32),
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn adjacent(pos: u32, start: u32, end: u32) -> bool {
    if start > 0 {
        if pos >= start - 1 && pos <= end + 1 {
            return true;
        }
    } else {
        if pos >= start && pos <= end + 1 {
            return true;
        }
    }

    false
}

fn is_part_number(
    line: u32,
    max_lines: u32,
    start: u32,
    end: u32,
    parts: &Vec<Vec<(u32, u32)>>,
) -> bool {
    let start_line = if line > 1 { line - 1 } else { line };

    let end_line = if line == max_lines - 1 {
        line
    } else {
        line + 1
    };

    for i in start_line..(end_line + 1) {
        let p = &(parts[i as usize]);

        let adj = p.iter().filter(|(_, k)| adjacent(*k, start, end)).count();
        if adj > 0 {
            return true;
        }
    }

    false
}

fn main() {
    let mut part_numbers: Vec<Vec<PartNumber>> = Vec::new();
    let mut parts: Vec<Vec<(u32, u32)>> = Vec::new();

    let Ok(lines) = read_lines("./input.txt") else {
        panic!("couldn't read input");
    };

    let mut line_num: u32 = 0;

    for l in lines {
        let line = l.unwrap();

        let mut buff: Vec<char> = Vec::new();
        let mut start: u32 = 0;

        let mut chars = line.chars();
        let mut char_num: u32 = 0;

        let mut this_row: Vec<PartNumber> = Vec::new();
        let mut this_parts: Vec<(u32, u32)> = Vec::new();

        while let Some(c) = chars.next() {
            if c.is_digit(10) {
                if buff.len() == 0 {
                    start = char_num;
                }
                buff.push(c);
            } else {
                if buff.len() > 0 {
                    let number = buff.iter().collect::<String>();
                    let x = u32::from_str_radix(&number, 10).unwrap();
                    this_row.push(PartNumber {
                        number: x,
                        start: (line_num, start),
                        end: (line_num, char_num - 1),
                    });
                }

                buff.clear();

                if c != '.' {
                    this_parts.push((line_num, char_num));
                }
            }

            char_num = char_num + 1;
        }

        line_num = line_num + 1;
        part_numbers.push(this_row);
        parts.push(this_parts);
    }

    let result: u32 = part_numbers
        .iter()
        .map(|pn| {
            pn.iter()
                .filter(|p| {
                    if is_part_number(p.start.0, line_num, p.start.1, p.end.1, &parts) {
                        true
                    } else {
                        false
                    }
                })
                .map(|p| p.number)
                .sum::<u32>()
        })
        .sum::<u32>();

    println!("{}", result);
}

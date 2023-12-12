use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const INSERT_SIZE: i64 = 999999;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Clone, Debug, PartialEq)]
enum Object {
    Galaxy,
}

// (0, 4) -> (10, 9) = 15
// (6, 1) -> (11, 5) = 9
fn distance(
    x: (usize, usize),
    y: (usize, usize),
    empty_rows: &HashSet<usize>,
    empty_columns: &HashSet<usize>,
) -> i64 {
    let mut points = vec![(x.0 as i64, x.1 as i64), (y.0 as i64, y.1 as i64)];
    points.sort_by(|a, b| a.0.cmp(&b.0));

    let m = points[0];
    let n = points[1];

    let expanded_rows = empty_rows
        .iter()
        .filter(|i| {
            // println!("filter row {} {} {}", **i, m.0, n.0);
            (**i as i64) > m.0 && (**i as i64) < n.0
        })
        .collect::<Vec<&usize>>()
        .len();

    let rn = (n.0 as i64 + expanded_rows as i64 * INSERT_SIZE, n.1);
    points[1] = rn;
    points.sort_by(|a, b| a.1.cmp(&b.1));

    let m = points[0];
    let n = points[1];

    let expanded_cols = empty_columns
        .iter()
        .filter(|i| {
            // println!("filter col {} {} {}", **i, m.1, n.1);
            (**i as i64) > m.1 && (**i as i64) < n.1
        })
        .collect::<Vec<&usize>>()
        .len();

    // println!("{}, {}", expanded_rows, expanded_cols);

    let cn = (n.0, n.1 as i64 + expanded_cols as i64 * INSERT_SIZE);
    // println!("{:?} + {:?}", n, en);

    let d = (m.0 - cn.0).abs() + (m.1 - cn.1).abs();
    // println!("{:?} -> {:?}: {}", m, en, d);
    d
}

fn main() {
    let Ok(lines) = read_lines("./input.txt") else {
        panic!("couldn't read input");
    };

    let mut grid: Vec<Vec<Option<Object>>> = Vec::new();
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    let mut empty_columns: HashSet<usize> = HashSet::new();
    let mut empty_rows: HashSet<usize> = HashSet::new();
    let mut line_number: usize = 0;

    for l in lines {
        let mut row: Vec<Option<Object>> = Vec::new();
        let mut found_planet = false;

        let line = l.unwrap();
        line.chars().for_each(|c| {
            if c == '.' {
                row.push(None);
            } else {
                row.push(Some(Object::Galaxy));
                found_planet = true;
            }
        });

        if !found_planet {
            println!("Empty row: {}", line_number);
            empty_rows.insert(line_number);
        }
        grid.push(row);
        line_number = line_number + 1;
    }

    let cols = grid[0].len();
    let rows = grid.len();

    println!("{},{}", rows, cols);

    println!("Expanding cols");

    for j in 0..cols {
        let mut empty_col = true;

        for i in 0..rows {
            if grid[i][j] == Some(Object::Galaxy) {
                empty_col = false;
            }
        }

        if empty_col {
            println!("Empty col {}", j);
            empty_columns.insert(j);
        }
    }

    // for i in 0..line_number {
    //     println!("{:?}", grid[i]);
    // }
    // println!("{},{}", line_number, grid[0].len());

    println!("Finding galaxies");
    // Finished setup
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == Some(Object::Galaxy) {
                galaxies.push((i, j));
            }
        }
    }

    println!("Calculating distances");

    let distances = galaxies
        .into_iter()
        .tuple_combinations()
        .map(|(x, y)| distance(x, y, &empty_rows, &empty_columns))
        .collect::<Vec<i64>>();
    // println!("{}", distances.len());
    // println!("{:?}", distances);
    let result = distances.iter().sum::<i64>();
    println!("{}", result);
}

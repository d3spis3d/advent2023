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

    let mut predictions: Vec<i64> = Vec::new();

    for l in lines {
        let line = l.unwrap();

        let initial = line
            .split(" ")
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let mut dataset: Vec<Vec<i64>> = vec![initial];
        let mut index: usize = 0;

        loop {
            let mut next: Vec<i64> = Vec::new();
            for i in 0..dataset[index].len() - 1 {
                next.push(dataset[index][i + 1] - dataset[index][i]);
            }

            if next.iter().all(|x| *x == 0) {
                next.insert(0, 0);
                dataset.push(next);
                break;
            } else {
                dataset.push(next);
                index = index + 1;
            }
        }

        for i in (0..dataset.len() - 1).rev() {
            let l = dataset[i].first().unwrap();
            let n = dataset[i + 1].first().unwrap();
            let x = *l - *n;
            dataset[i].insert(0, x);
        }

        // println!("{:?}", dataset);
        // println!("----------------");
        predictions.push(*dataset[0].first().unwrap());
    }

    let result = predictions.iter().sum::<i64>();
    println!("{}", result);
}

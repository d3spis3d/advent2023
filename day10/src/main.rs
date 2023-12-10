use std::collections::VecDeque;
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

#[derive(Debug, PartialEq, Copy, Clone)]
enum Pipe {
    Horiz,
    Vertical,
    L,
    F,
    J,
    Seven,
    Empty,
    Start,
}

impl Pipe {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '-' => Self::Horiz,
            '|' => Self::Vertical,
            'L' => Self::L,
            'F' => Self::F,
            'J' => Self::J,
            '7' => Self::Seven,
            'S' => Self::Start,
            _ => panic!("bad pipe"),
        }
    }

    fn match_up(self: &Self, other: &Pipe) -> bool {
        match self {
            Pipe::Vertical | Pipe::L | Pipe::J | Pipe::Start => match other {
                Pipe::Vertical | Pipe::F | Pipe::Seven | Pipe::Start => true,
                _ => false,
            },
            Pipe::Horiz | Pipe::F | Pipe::Seven => false,
            _ => panic!("not up"),
        }
    }

    fn match_right(self: &Self, other: &Pipe) -> bool {
        match self {
            Pipe::F | Pipe::L | Pipe::Start | Pipe::Horiz => match other {
                Pipe::Horiz | Pipe::J | Pipe::Seven | Pipe::Start => true,
                _ => false,
            },
            Pipe::Vertical | Pipe::J | Pipe::Seven => false,
            _ => panic!("not right"),
        }
    }

    fn match_down(self: &Self, other: &Pipe) -> bool {
        match self {
            Pipe::F | Pipe::Seven | Pipe::Start | Pipe::Vertical => match other {
                Pipe::L | Pipe::J | Pipe::Vertical | Pipe::Start => true,
                _ => false,
            },
            Pipe::J | Pipe::L | Pipe::Horiz => false,
            _ => panic!("not right"),
        }
    }

    fn match_left(self: &Self, other: &Pipe) -> bool {
        match self {
            Pipe::J | Pipe::Seven | Pipe::Start | Pipe::Horiz => match other {
                Pipe::L | Pipe::F | Pipe::Horiz | Pipe::Start => true,
                _ => false,
            },
            Pipe::F | Pipe::L | Pipe::Vertical => false,
            _ => panic!("not right"),
        }
    }
}

fn main() {
    let Ok(lines) = read_lines("./input.txt") else {
        panic!("couldn't read input");
    };

    let mut grid: Vec<Vec<Pipe>> = Vec::new();
    let mut start: Option<(usize, usize)> = None;

    let mut line_number: usize = 0;

    for l in lines {
        let mut row: Vec<Pipe> = Vec::new();

        let line = l.unwrap();

        line.chars().enumerate().for_each(|(j, c)| {
            let p = Pipe::from(c);
            if p == Pipe::Start {
                start = Some((line_number, j));
            }
            row.push(p);
        });

        grid.push(row);
        line_number = line_number + 1;
    }

    let rows = line_number;
    let columns = grid[0].len();

    // println!("{:?}", grid);
    // println!("{:?}", start);
    //
    let (s_i, s_j) = start.unwrap();

    let mut pos: (usize, usize, Pipe) = (s_i, s_j, Pipe::Start);
    let mut prev: (usize, usize, Pipe) = pos.clone();
    let mut dq: VecDeque<(usize, usize)> = VecDeque::new();

    loop {
        let (i, j, pipe) = pos;

        if i != 0 {
            if pipe.match_up(&grid[i - 1][j]) {
                let new_i = i - 1;
                let new_j = j;
                let new_pos = &(new_i, new_j, grid[new_i][new_j]);
                if (new_pos.0, new_pos.1) != (prev.0, prev.1) {
                    // println!("found next {:?}", new_pos);
                    dq.push_back((new_i, new_j));
                    prev = pos.clone();
                    pos = new_pos.clone();

                    if (new_i, new_j) == (s_i, s_j) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
        }

        if j != rows - 1 {
            if pipe.match_right(&grid[i][j + 1]) {
                let new_i = i;
                let new_j = j + 1;
                let new_pos = (new_i, new_j, grid[new_i][new_j]);
                if (new_pos.0, new_pos.1) != (prev.0, prev.1) {
                    // println!("found next {:?}", new_pos);
                    dq.push_back((new_i, new_j));
                    prev = pos.clone();
                    pos = new_pos.clone();

                    if (new_i, new_j) == (s_i, s_j) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
        }

        if i != columns - 1 {
            if pipe.match_down(&grid[i + 1][j]) {
                let new_i = i + 1;
                let new_j = j;
                let new_pos = &(new_i, new_j, grid[new_i][new_j]);
                if (new_pos.0, new_pos.1) != (prev.0, prev.1) {
                    // println!("found next {:?}", new_pos);
                    dq.push_back((new_i, new_j));
                    prev = pos.clone();
                    pos = new_pos.clone();

                    if (new_i, new_j) == (s_i, s_j) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
        }

        if j != 0 {
            if pipe.match_left(&grid[i][j - 1]) {
                let new_i = i;
                let new_j = j - 1;
                let new_pos = &(new_i, new_j, grid[new_i][new_j]);
                if (new_pos.0, new_pos.1) != (prev.0, prev.1) {
                    // println!("found next {:?}", new_pos);
                    dq.push_back((new_i, new_j));
                    prev = pos.clone();
                    pos = new_pos.clone();

                    if (new_i, new_j) == (s_i, s_j) {
                        break;
                    } else {
                        continue;
                    }
                }
            };
        }
    }

    // println!("{:?}", dq);
    println!("Result: {}", dq.len() / 2);
}

use reformation::Reformation;
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

#[derive(Reformation, Debug)]
#[reformation(r"Game {id}: {info}")]
struct Game {
    id: u32,
    info: String,
}

#[derive(Reformation, Debug)]
#[reformation(r"{num} {colour}")]
struct ColourSet {
    num: u32,
    colour: String,
}

fn main() {
    let Ok(lines) = read_lines("./input.txt") else {
        panic!("couldn't read input");
    };

    let max_cubes: HashMap<String, u32> = HashMap::from([
        (String::from("red"), 12),
        (String::from("green"), 13),
        (String::from("blue"), 14),
    ]);

    let mut games_possible: Vec<u32> = Vec::new();

    for l in lines {
        let line = l.unwrap();
        let game = Game::parse(&line).unwrap();

        let mut game_possible = true;

        game.info.split("; ").for_each(|d| {
            d.split(", ").for_each(|c| {
                let set = ColourSet::parse(c).unwrap();
                let max = max_cubes.get(&set.colour).unwrap();
                if set.num > *max {
                    game_possible = false;
                }
            })
        });

        if game_possible {
            games_possible.push(game.id);
        }
    }

    let result: u32 = games_possible.iter().sum();
    println!("{}", result);
}

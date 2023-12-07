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

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from(c: char) -> Option<Self> {
        match c {
            '2' => Some(Card::Two),
            '3' => Some(Card::Three),
            '4' => Some(Card::Four),
            '5' => Some(Card::Five),
            '6' => Some(Card::Six),
            '7' => Some(Card::Seven),
            '8' => Some(Card::Eight),
            '9' => Some(Card::Nine),
            'T' => Some(Card::Ten),
            'J' => Some(Card::Jack),
            'Q' => Some(Card::Queen),
            'K' => Some(Card::King),
            'A' => Some(Card::Ace),
            _ => None,
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    HighCard(Card),
    Pair(Card),
    TwoPair(Card, Card),
    Three(Card),
    FullHouse(Card, Card),
    Four(Card),
    Five(Card),
}

impl HandType {
    fn from(collection: HashMap<Card, u8>) -> Self {
        let mut contents = collection.into_iter().collect::<Vec<(Card, u8)>>();
        contents.sort_by(|a, b| b.1.cmp(&a.1));

        let (card, num) = &contents[0];
        match *num {
            5 => return HandType::Five(*card),
            4 => return HandType::Four(*card),
            3 => {
                let (c2, n2) = &contents[1];
                if *n2 == 2 {
                    return HandType::FullHouse(*card, *c2);
                } else {
                    return HandType::Three(*card);
                }
            }
            2 => {
                let (c2, n2) = &contents[1];
                if *n2 == 2 {
                    let mut pairs = Vec::from([*card, *c2]);
                    pairs.sort();
                    pairs.reverse();
                    return HandType::TwoPair(pairs[0], pairs[1]);
                } else {
                    return HandType::Pair(*card);
                }
            }
            1 => {
                let mut cards = contents.into_iter().map(|(c, _)| c).collect::<Vec<Card>>();
                cards.sort();
                cards.reverse();
                return HandType::HighCard(cards[0]);
            }
            _ => panic!("argggggh!"),
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    wager: u64,
}

impl Hand {
    fn from(s: String) -> Self {
        let mut parts = s.split(" ");
        let card_string = parts.next().unwrap();
        let wager = parts.next().unwrap().parse::<u64>().unwrap();

        let cards = card_string
            .chars()
            .map(|c| Card::from(c).unwrap())
            .collect::<Vec<Card>>();

        let card_count: HashMap<Card, u8> = cards.iter().fold(HashMap::new(), |mut acc, c| {
            acc.entry(*c)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
            acc
        });

        let hand_type = HandType::from(card_count);

        Hand {
            cards: cards,
            hand_type: hand_type,
            wager: wager,
        }
    }
}

fn main() {
    let Ok(lines) = read_lines("./test.txt") else {
        panic!("couldn't read input");
    };

    for l in lines {
        let line = l.unwrap();
        let hand = Hand::from(line);
        println!("{:?}", hand);
    }
}

use std::{
    fmt::{Debug, Write},
    io,
};

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 6440, "test input failed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(path: &str) -> i64 {
    let mut input = Input::parse(path);

    input.hands.sort_by_key(|f| f.0);

    let mut score = 0;
    for (rank, &(_hand, bid)) in input.hands.iter().enumerate() {
        // rank starts at 1 in the puzzle spec, so add one
        score += bid * (rank as i64 + 1);
    }

    score
}

#[derive(Debug)]
struct Input {
    hands: Vec<(Hand, i64)>,
}

impl Input {
    fn parse(path: &str) -> Self {
        let mut result = Input { hands: Vec::new() };

        // no Result return type, since out input from AOC should always be valid
        let input = read_lines(path).unwrap();

        for line in input {
            let line = line.unwrap();

            let (hand_text, bid_text) = line.split_once(' ').unwrap();

            let hand = Hand::from_text(hand_text);
            let bid = bid_text.parse().unwrap();

            result.hands.push((hand, bid));
        }

        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Hand {
    FiveOAK(Card),
    FourOAK((Card, Card)),             // the four-of card, then the one-of card
    FullHouse((Card, Card)),           // the three-of card, then then two-of
    ThreeOfAKind((Card, Card, Card)),  // the three-of, the high one, then low one
    TwoPair((Card, Card, Card)),       // the higher two-of, the lower two-of, then the one-of
    OnePair((Card, Card, Card, Card)), // two-of, highest one-of to lowest
    HighCard((Card, Card, Card, Card, Card)), // highest to lowest card
}

impl Hand {
    fn from_text(input: &str) -> Self {
        assert!(input.len() == 5);

        let mut cards = Vec::with_capacity(5);

        for c in input.chars() {
            let this_card = Card::from_char(c).unwrap();

            let existing = cards.iter_mut().find(|(_, card)| *card == this_card);
            match existing {
                Some((count, _)) => *count += 1,
                None => cards.push((1, this_card)),
            }
        }

        // order by count, and then card value, highest to lowest (so descending order)
        cards.sort_by(|a, b| b.cmp(a));

        match cards.as_slice() {
            [(5, a)] => Hand::FiveOAK(*a),
            [(4, a), (1, b)] => Hand::FourOAK((*a, *b)),
            [(3, a), (2, b)] => Hand::FullHouse((*a, *b)),
            [(3, a), (1, b), (1, c)] => Hand::ThreeOfAKind((*a, *b, *c)),
            [(2, a), (2, b), (1, c)] => Hand::TwoPair((*a, *b, *c)),
            [(2, a), (1, b), (1, c), (1, d)] => Hand::OnePair((*a, *b, *c, *d)),
            [(1, a), (1, b), (1, c), (1, d), (1, e)] => Hand::HighCard((*a, *b, *c, *d, *e)),
            _ => unreachable!(),
        }
    }

    // stronger types (five of a kind) have higher strength values. Each enum value has a unique
    // strength
    fn type_strength(&self) -> i8 {
        match self {
            Hand::FiveOAK(_) => 6,
            Hand::FourOAK(_) => 5,
            Hand::FullHouse(_) => 4,
            Hand::ThreeOfAKind(_) => 3,
            Hand::TwoPair(_) => 2,
            Hand::OnePair(_) => 1,
            Hand::HighCard(_) => 0,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // order by strength first. only need to check face values if strength is the same
        let strength = self.type_strength().cmp(&other.type_strength());
        match strength {
            std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => (),
        };

        match (self, other) {
            (Hand::FiveOAK(l), Hand::FiveOAK(r)) => l.cmp(r),
            (Hand::FourOAK(l), Hand::FourOAK(r)) => l.cmp(r),
            (Hand::FullHouse(l), Hand::FullHouse(r)) => l.cmp(r),
            (Hand::ThreeOfAKind(l), Hand::ThreeOfAKind(r)) => l.cmp(r),
            (Hand::TwoPair(l), Hand::TwoPair(r)) => l.cmp(r),
            (Hand::OnePair(l), Hand::OnePair(r)) => l.cmp(r),
            (Hand::HighCard(l), Hand::HighCard(r)) => l.cmp(r),
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Card {
    // 13 values. 0 is card '2', 12 is 'Ace'
    val: u8,
}

impl Card {
    const CHARS: &[u8] = "123456789TJQKA".as_bytes();

    fn from_char(input: char) -> Option<Self> {
        for (i, c) in Self::CHARS.iter().enumerate() {
            if input == *c as char {
                return Some(Card { val: i as u8 });
            }
        }

        None
    }

    fn to_char(self) -> char {
        Self::CHARS[self.val as usize].into()
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.to_char())
    }
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}

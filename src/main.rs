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
enum HandType {
    FiveOAK,
    FourOAK,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn strength(&self) -> i8 {
        match self {
            HandType::FiveOAK => 6,
            HandType::FourOAK => 5,
            HandType::FullHouse => 4,
            HandType::ThreeOfAKind => 3,
            HandType::TwoPair => 2,
            HandType::OnePair => 1,
            HandType::HighCard => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    poker_type: HandType,
}

impl Hand {
    fn from_text(input: &str) -> Self {
        assert!(input.len() == 5);

        let mut cards = [Card::default(); 5];
        let mut cards_histo = Vec::with_capacity(5);

        for (i, c) in input.chars().enumerate() {
            let this_card = Card::from_char(c).unwrap();
            cards[i] = this_card;

            let existing = cards_histo.iter_mut().find(|(_, card)| *card == this_card);
            match existing {
                Some((count, _)) => *count += 1,
                None => cards_histo.push((1, this_card)),
            }
        }

        // order by count, and then card value, highest to lowest (so descending order)
        cards_histo.sort_by(|a, b| b.cmp(a));

        let poker_type = match cards_histo.as_slice() {
            [(5, _)] => HandType::FiveOAK,
            [(4, _), (1, _)] => HandType::FourOAK,
            [(3, _), (2, _)] => HandType::FullHouse,
            [(3, _), (1, _), (1, _)] => HandType::ThreeOfAKind,
            [(2, _), (2, _), (1, _)] => HandType::TwoPair,
            [(2, _), (1, _), (1, _), (1, _)] => HandType::OnePair,
            [(1, _), (1, _), (1, _), (1, _), (1, _)] => HandType::HighCard,
            _ => unreachable!(),
        };

        Self { cards, poker_type }
    }

    // stronger types (five of a kind) have higher strength values. Each enum value has a unique
    // strength
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // order by strength first. only need to check face values if strength is the same
        let strength = self.poker_type.strength().cmp(&other.poker_type.strength());
        match strength {
            std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => (),
        };

        self.cards.cmp(&other.cards)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
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

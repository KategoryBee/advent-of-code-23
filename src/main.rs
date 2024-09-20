use std::{
    fmt::{Debug, Write},
    io,
};

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 5905, "test input failed");

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
    fn from_cards(cards: &[Card; 5]) -> Self {
        let mut cards_histo = [0u8; Card::COUNT];
        let mut jokers = 0;

        for c in cards {
            if *c == Card::JOKER {
                jokers += 1;
            } else {
                cards_histo[c.val as usize] += 1;
            }
        }

        // order by reverse count. we only care about the 'shape' of the histogram.
        cards_histo.sort_by(|a, b| b.cmp(a));
        cards_histo[0] += jokers;

        match cards_histo.as_slice() {
            [5, ..] => HandType::FiveOAK,
            [4, 1, ..] => HandType::FourOAK,
            [3, 2, ..] => HandType::FullHouse,
            [3, 1, 1, ..] => HandType::ThreeOfAKind,
            [2, 2, 1, ..] => HandType::TwoPair,
            [2, 1, 1, 1, ..] => HandType::OnePair,
            [1, 1, 1, 1, 1, ..] => HandType::HighCard,
            _ => unreachable!(),
        }
    }

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

        for (i, c) in input.chars().enumerate() {
            cards[i] = Card::from_char(c).unwrap();
        }
        let poker_type = HandType::from_cards(&cards);

        Self { cards, poker_type }
    }
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
    const JOKER: Card = Card { val: 0 };
    const CHARS: &[u8] = "J23456789TQKA".as_bytes();
    const COUNT: usize = 13;

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

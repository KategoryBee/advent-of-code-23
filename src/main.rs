use std::{
    collections::{HashMap, HashSet},
    io,
};

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 30, "test input failed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(path: &str) -> i32 {
    let input = read_lines(path).unwrap();

    let mut cards_count: HashMap<usize, i32> = HashMap::new();
    let mut winning_numbers: HashSet<i32> = HashSet::with_capacity(20);

    for (card_num, line) in input.enumerate() {
        winning_numbers.clear();

        let mut num_current_card = *cards_count.get(&card_num).unwrap_or(&0);
        num_current_card += 1;
        cards_count.insert(card_num, num_current_card);

        let line = line.unwrap();
        let after_card = line.split_once(':').unwrap().1;
        let (winning, have) = after_card.split_once('|').unwrap();

        for w in winning.split_whitespace() {
            winning_numbers.insert(w.parse().unwrap());
        }

        let mut bonus_card_amount = 0;
        for a in have.split_whitespace() {
            let num: i32 = a.parse().unwrap();
            if winning_numbers.contains(&num) {
                bonus_card_amount += 1;
            }
        }

        for bonus_card in 0..bonus_card_amount {
            let bonus_card_num = card_num + bonus_card + 1;

            let current_count = *cards_count.get(&bonus_card_num).unwrap_or(&0);
            cards_count.insert(bonus_card_num, current_count + num_current_card);
        }
    }

    cards_count.values().sum()
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}

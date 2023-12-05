use std::{collections::HashSet, io};

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 13, "test input failed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(path: &str) -> i32 {
    let input = read_lines(path).unwrap();

    let mut solution = 0;

    let mut winning_numbers: HashSet<i32> = HashSet::with_capacity(20);

    for line in input {
        winning_numbers.clear();

        let line = line.unwrap();
        let after_card = line.split_once(':').unwrap().1;
        let (winning, have) = after_card.split_once('|').unwrap();

        for w in winning.split_whitespace() {
            winning_numbers.insert(w.parse().unwrap());
        }

        let mut points = 0;
        for a in have.split_whitespace() {
            let num: i32 = a.parse().unwrap();
            if winning_numbers.contains(&num) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }

        solution += points;
    }

    solution
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}

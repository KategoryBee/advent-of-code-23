use std::{collections::HashSet, io};

use regex::bytes::Regex;

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 4361, "test input failed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn is_symbol(b: u8) -> bool {
    !(b.is_ascii_digit() || b == b'.')
}

fn symbol_locations(path: &str) -> HashSet<(i32, i32)> {
    let mut res = HashSet::new();

    let input = read_lines(path).unwrap();

    for (row, line) in input.enumerate() {
        let line = line.unwrap();
        for (col, character) in line.bytes().enumerate() {
            if is_symbol(character) {
                res.insert((row as i32, col as i32));
            }
        }
    }

    res
}

fn solve(path: &str) -> i32 {
    let mut solution = 0;

    let next_number = Regex::new(r"\d+").unwrap();

    let locations = symbol_locations(path);

    let input = read_lines(path).unwrap();

    for (row, line) in input.enumerate() {
        let line = line.unwrap();
        let line = line.as_bytes();

        let mut offset = 0;
        loop {
            let Some(caps) = next_number.captures_at(line, offset) else {
                // no more numbers in line
                break;
            };

            let digits = caps.get(0).unwrap();

            let mut has_symbol = false;
            for r in (row as i32 - 1)..=(row as i32 + 1) {
                for c in (digits.start() as i32 - 1)..=(digits.end() as i32) {
                    if locations.contains(&(r, c)) {
                        has_symbol = true;
                    }
                }
            }

            if has_symbol {
                let digits_str = std::str::from_utf8(digits.as_bytes()).unwrap();
                let digits_val: i32 = digits_str.parse().unwrap();
                solution += digits_val;
            }

            offset = digits.end();
        }
    }

    solution
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}

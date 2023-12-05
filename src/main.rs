use std::{collections::HashMap, io};

use regex::bytes::Regex;

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 467835, "test input failed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn is_gear(b: u8) -> bool {
    b == b'*'
}

struct Gear {
    ratio: i32,
    connected_count: i32,
}

impl Default for Gear {
    fn default() -> Self {
        Self {
            ratio: 1,
            connected_count: 0,
        }
    }
}

fn gear_locations(path: &str) -> HashMap<(i32, i32), Gear> {
    let mut res = HashMap::new();

    let input = read_lines(path).unwrap();

    for (row, line) in input.enumerate() {
        let line = line.unwrap();
        for (col, character) in line.bytes().enumerate() {
            if is_gear(character) {
                res.insert((row as i32, col as i32), Gear::default());
            }
        }
    }

    res
}

fn solve(path: &str) -> i32 {
    let next_number = Regex::new(r"\d+").unwrap();

    let mut gears = gear_locations(path);

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

            let digits_str = std::str::from_utf8(digits.as_bytes()).unwrap();
            let digits_val: i32 = digits_str.parse().unwrap();

            for r in (row as i32 - 1)..=(row as i32 + 1) {
                for c in (digits.start() as i32 - 1)..=(digits.end() as i32) {
                    if let Some(gear) = gears.get_mut(&(r, c)) {
                        gear.connected_count += 1;
                        gear.ratio *= digits_val;
                    }
                }
            }

            offset = digits.end();
        }
    }

    let mut solution = 0;
    for (_loc, gear) in gears {
        if gear.connected_count > 1 {
            solution += gear.ratio;
        }
    }

    solution
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}

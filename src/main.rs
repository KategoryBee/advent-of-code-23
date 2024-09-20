use std::{collections::HashMap, fmt::Debug, io};

use regex::Regex;

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 6, "test input failed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(path: &str) -> i64 {
    let input = Input::parse(path);

    let mut at = "AAA";
    let mut step = 0;

    while at != "ZZZ" {
        let direction = input.directions[step % input.directions.len()];
        let choices = &input.network[at];

        step += 1;

        if direction == b'L' {
            at = &choices.0;
        } else {
            at = &choices.1;
        }
    }

    step as i64
}

#[derive(Debug)]
struct Input {
    directions: Vec<u8>,
    network: HashMap<String, (String, String)>,
}

impl Input {
    fn parse(path: &str) -> Self {
        // no Result return type, since out input from AOC should always be valid
        let mut input = read_lines(path).unwrap();

        let node_style = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

        let steps = input.next().unwrap().unwrap();
        let mut network = HashMap::new();

        for line in input {
            let line = line.unwrap();
            if line.is_empty() {
                continue;
            }

            let caps = node_style.captures(&line).unwrap();

            let name = caps[1].to_owned();
            let to_l = caps[2].to_owned();
            let to_r = caps[3].to_owned();

            network.insert(name, (to_l, to_r));
        }

        Input {
            directions: steps.into_bytes(),
            network,
        }
    }
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}

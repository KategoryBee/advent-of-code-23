use std::io;

use regex::Regex;

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 35, "test input failed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(path: &str) -> i64 {
    let num_capture = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();

    let mut input = read_lines(path).unwrap();

    let mut current_stage: Vec<i64> = {
        // Read in seed list
        let seeds_str = input.next().unwrap().unwrap();
        let seeds_str = seeds_str.strip_prefix("seeds: ").unwrap();

        seeds_str
            .split_ascii_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect()
    };

    let mut next_stage = current_stage.clone();

    for line in input {
        let line = line.unwrap();

        if line.ends_with("map:") {
            current_stage.clone_from(&next_stage);
            continue;
        }

        let Some(digits) = num_capture.captures(&line) else {
            assert!(line.is_empty());
            continue;
        };

        let dest_start: i64 = digits[1].parse().unwrap();
        let source_start: i64 = digits[2].parse().unwrap();
        let range_len: i64 = digits[3].parse().unwrap();

        let source = source_start..(source_start + range_len);

        for (i, &v) in current_stage.iter().enumerate() {
            if source.contains(&v) {
                let offset = v - source_start;
                next_stage[i] = dest_start + offset;
                println!("{} to {}", v, dest_start + offset);
            }
        }
    }

    dbg!(&next_stage);
    *next_stage.iter().min().unwrap()
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}

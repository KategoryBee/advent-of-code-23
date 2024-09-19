use std::io;

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 288, "test input failed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(path: &str) -> i64 {
    let input = Input::parse(path);
    let mut result = 1;

    for (time, distance) in input.times.into_iter().zip(input.distances) {
        let mut winnable = 0;

        for hold in 0..time {
            let travelled = hold * (time - hold);
            if travelled > distance {
                winnable += 1
            }
        }

        result *= winnable;
    }

    result
}

struct Input {
    times: Vec<i64>,
    distances: Vec<i64>,
}

impl Input {
    fn parse(path: &str) -> Self {
        // no Result return type, since out input from AOC should always be valid
        let mut input = read_lines(path).unwrap();

        let times: Vec<i64> = {
            let times_str = input.next().unwrap().unwrap();
            let times_str = times_str.strip_prefix("Time:").unwrap();

            times_str
                .split_ascii_whitespace()
                .map(|a| a.parse::<i64>().unwrap())
                .collect()
        };

        let distances: Vec<i64> = {
            let dist_str = input.next().unwrap().unwrap();
            let dist_str = dist_str.strip_prefix("Distance:").unwrap();

            dist_str
                .split_ascii_whitespace()
                .map(|a| a.parse::<i64>().unwrap())
                .collect()
        };

        assert!(times.len() == distances.len());
        Self { times, distances }
    }
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}

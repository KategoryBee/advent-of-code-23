use std::io;

fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 71503, "test input failed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(path: &str) -> i64 {
    // This solution assumes the input files have been manually edited to fix the 'kerning' issue.
    // Was easier than writing code.
    let input = Input::parse(path);

    let mut winnable = 0;

    // An ideal solution would probably binary search the space to find out the first and last
    // 'hold' time that becomes and winner. But the problem space is small enough (only 54946592
    // values on my input) that it's faster to brute force than to code it. Even writing this
    // commment took longer than the debug run of the program
    for hold in 0..input.time {
        let travelled = hold * (input.time - hold);
        if travelled > input.distance {
            winnable += 1
        }
    }

    winnable
}

struct Input {
    time: i64,
    distance: i64,
}

impl Input {
    fn parse(path: &str) -> Self {
        // no Result return type, since out input from AOC should always be valid
        let mut input = read_lines(path).unwrap();

        let time: i64 = {
            let times_str = input.next().unwrap().unwrap();
            let times_str = times_str.strip_prefix("Time: ").unwrap();
            times_str.parse().unwrap()
        };

        let distance: i64 = {
            let dist_str = input.next().unwrap().unwrap();
            let dist_str = dist_str.strip_prefix("Distance: ").unwrap();
            dist_str.parse().unwrap()
        };

        Self { time, distance }
    }
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}

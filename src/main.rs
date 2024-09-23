use std::io;
fn main() {
    let test_result = solve("test.txt");
    assert_eq!(test_result, 2, "test input failed");
    println!("Test passed");

    let result = solve("input.txt");
    println!("result: {result}");
}

fn solve(path: &str) -> i64 {
    let mut input = Input::parse(path);

    input
        .puzzles
        .iter_mut()
        .map(|x| {
            x.values.reverse();
            solve_step(&x.values)
        })
        .sum()
}

fn solve_step(input: &[i64]) -> i64 {
    assert!(input.len() > 1);

    if input.iter().all(|v| *v == 0) {
        return 0;
    }

    let differences: Vec<i64> = input.windows(2).map(|v| v[1] - v[0]).collect();

    let diff = solve_step(&differences);

    input.last().unwrap() + diff
}

#[derive(Debug)]
struct Input {
    puzzles: Vec<Puzzle>,
}

#[derive(Debug)]
struct Puzzle {
    values: Vec<i64>,
}

impl Input {
    // no Result return type, since out input from AOC should always be valid
    fn parse(path: &str) -> Self {
        let input = read_lines(path).unwrap();

        let line_to_puzzle = |line: io::Result<String>| {
            let values = line
                .unwrap()
                .split_ascii_whitespace()
                .map(|y| y.parse().unwrap())
                .collect();
            Puzzle { values }
        };

        let puzzles = input.map(line_to_puzzle).collect();

        Input { puzzles }
    }
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}

use std::io;

use num::Integer;

fn main() {
    let test_result = solve("test.txt", Dir::Right);
    assert_eq!(test_result, 8, "test input failed");
    println!("Test passed");

    let result = solve("input.txt", Dir::Left);
    println!("result: {result}");
}

fn solve(path: &str, initial_dir: Dir) -> i64 {
    let input = Input::parse(path);

    let mut steps = 0;
    let mut pos = input.start;

    let mut dir = initial_dir;

    // The initial step doesn't specify its connections. we _could_ work it out from the surrounds,
    // but since we only have 1 puzzle to solve we pass it in and special case.
    pos = do_move(pos, dir);
    steps += 1;

    while pos != input.start {
        let pipe = input.field[pos.1][pos.0];

        let new_dir = match (pipe, dir) {
            (b'-', dir) => dir,
            (b'|', dir) => dir,
            (b'L', Dir::Left) => Dir::Up,
            (b'L', Dir::Down) => Dir::Right,
            (b'J', Dir::Right) => Dir::Up,
            (b'J', Dir::Down) => Dir::Left,
            (b'7', Dir::Right) => Dir::Down,
            (b'7', Dir::Up) => Dir::Left,
            (b'F', Dir::Up) => Dir::Right,
            (b'F', Dir::Left) => Dir::Down,
            _ => panic!(),
        };

        // move.
        dir = new_dir;
        pos = do_move(pos, dir);

        steps += 1;
    }

    // We're on a grid, so this _should_ be even, always. but if it's not i need to think through
    // the logic
    assert!(steps.is_even());

    steps / 2
}

fn do_move(pos: (usize, usize), dir: Dir) -> (usize, usize) {
    match dir {
        Dir::Up => (pos.0, pos.1 - 1),
        Dir::Down => (pos.0, pos.1 + 1),
        Dir::Left => (pos.0 - 1, pos.1),
        Dir::Right => (pos.0 + 1, pos.1),
    }
}

#[derive(Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Input {
    // x, y. 0, 0 is top left
    start: (usize, usize),
    // outer is row(y), inner is column(x)
    field: Vec<Vec<u8>>,
}

impl Input {
    // no Result return type, since out input from AOC should always be valid
    fn parse(path: &str) -> Self {
        let input = read_lines(path).unwrap();

        let line_to_inner = |line: io::Result<String>| line.unwrap().into_bytes();

        let field: Vec<Vec<u8>> = input.map(line_to_inner).collect();

        // TODO: find 'S'
        let mut start = None;
        for (row, row_data) in field.iter().enumerate() {
            for (col, col_data) in row_data.iter().enumerate() {
                if *col_data == b'S' {
                    start = Some((col, row));
                }
            }
        }

        Input {
            start: start.unwrap(),
            field,
        }
    }
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}
